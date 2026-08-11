use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::{HeapCons, HeapProd, HeapRb};
use rodio::Source;
use std::num::NonZero;

use crate::nes::F_CPU;

const SAMPLE_RATE: u32 = 48_000;
const BLIP_CAPACITY: u32 = SAMPLE_RATE / 10;

// How many frames the sound card takes at a time. rodio's own choice aims at
// 50ms, which measured here as the ring buffer emptying in swings of nearly
// 100ms and running dry roughly every other second. 512 frames is about 12ms on
// a 44.1kHz device; 256 measured no better and asks the card to wake twice as
// often. See `SoundEngine::open_sink`.
const DEVICE_PERIOD: u32 = 512;

// The sound card empties the ring buffer a whole period at a time, so the ring
// has to stay deeper than a period or it runs dry in between and the gaps are
// heard as clicks. Sized to cover several periods, since a device that misses
// its own deadline takes two at once. Every millisecond of it is also a
// millisecond of delay before a jump or a coin is heard, so it should not grow
// past what the periods need.
const RING_MS: usize = 100;
const RING_CAPACITY: usize = SAMPLE_RATE as usize * RING_MS / 1000;

// Where dynamic rate control aims to hold the buffer. Halfway leaves equal room
// for the card to fall behind or run ahead before either end is reached.
const TARGET_FILL: f32 = 0.5;

// The widest resampling correction, as a fraction of the nominal rate. What it
// has to absorb is the gap between the NES frame rate and the rate frames are
// actually produced at, plus whatever the sound card's nominal 48kHz is really
// running at. Both are fractions of a percent, but the correction has to
// comfortably exceed them or the buffer walks to an end and stays pinned there.
// A percent works out to about a sixth of a semitone, which is not audible on
// square waves.
const MAX_SKEW: f32 = 0.01;

// Smoothing on the fill estimate, as a per-frame weight. The level sawtooths
// across a third of the buffer on every audio period, and correcting against
// that would just feed the card's burstiness back into the resampler. The drift
// underneath it is a near-constant rate error, so the loop can afford to look
// at several seconds of history. ~4s here.
const FILL_SMOOTHING: f32 = 0.004;

pub struct APUSink {
    producer: HeapProd<i16>,
    blip: blip_buf::BlipBuf,
    last: i32,
    fill: f32,
}

impl APUSink {
    /// Ends a frame `frame_len` CPU clocks long and hands the resampled audio
    /// to the player. Clock 0 of the next frame is `frame_len` of this one.
    pub fn drain(&mut self, frame_len: u32) {
        self.blip.end_frame(frame_len).unwrap();

        while self.blip.samples_avail() > 0 {
            let temp = &mut [0i16; 1024];
            let count = self.blip.read_samples(temp, false);
            self.producer.push_slice(&temp[..count]);
        }
    }

    /// Records the APU output as `sample` from `clock` (CPU cycles since the
    /// start of the current frame) onwards. Only transitions cost anything, so
    /// calling this on every tick is cheap.
    pub fn push_sample(&mut self, clock: u32, sample: i32) {
        if sample != self.last {
            let _ = self.blip.add_delta(clock, sample - self.last);
            self.last = sample;
        }
    }

    /// Resamples slightly faster or slower so that what the emulator produces
    /// keeps pace with what the sound card consumes. Call once per frame, after
    /// `drain`.
    ///
    /// Frames are emitted on a wall-clock schedule that is close to the NES
    /// frame rate but never exactly it, so a fixed resampling ratio drifts until
    /// the buffer either runs dry or backs up and stalls emulation. Nudging the
    /// ratio towards whatever holds the buffer at `TARGET_FILL` makes the audio
    /// follow the frame schedule instead, which is what lets the schedule be
    /// chosen for smooth video rather than for the sound card's convenience.
    pub fn retune(&mut self) {
        let filled = RING_CAPACITY - self.vacant_len().min(RING_CAPACITY);
        let level = filled as f32 / RING_CAPACITY as f32;
        self.fill += (level - self.fill) * FILL_SMOOTHING;

        // Too full means we are outrunning the card, so ask blip for fewer
        // samples per CPU clock; too empty and we ask for more.
        let error = ((TARGET_FILL - self.fill) / TARGET_FILL).clamp(-1.0, 1.0);
        let rate = SAMPLE_RATE as f32 * (1.0 + MAX_SKEW * error);
        let _ = self.blip.set_rates(F_CPU as f64, rate as f64);
    }

    pub fn vacant_len(&self) -> usize {
        self.producer.vacant_len()
    }

    /// How many CPU clocks of emulation the ring buffer's free space can
    /// absorb, saturating at `MAX_QUERY` samples' worth. Accounts for blip's
    /// sub-sample phase, so gating on this has no rounding slop against a real
    /// frame length.
    pub fn clocks_free(&self) -> u32 {
        // blip measures time in 1/2^20 of a clock, so asking about 2^12 samples
        // or more overflows the u32 that count is multiplied into, and the
        // answer comes back wrapped: a nearly empty buffer reports as a nearly
        // full one, and a caller gating on it waits forever. Callers only need
        // to know whether one more frame fits, so stopping the question well
        // short of the overflow costs nothing.
        const MAX_QUERY: usize = 2048;

        let headroom = (BLIP_CAPACITY as usize).saturating_sub(self.blip.samples_avail() as usize);
        self.blip
            .clocks_needed(self.vacant_len().min(headroom).min(MAX_QUERY) as u32)
            .unwrap_or(0)
    }
}

pub struct APUSource {
    sample_buffer: HeapCons<i16>,
    last: i16,
}

impl APUSource {
    pub fn new() -> (Self, APUSink) {
        let mut blip = blip_buf::BlipBuf::new(BLIP_CAPACITY);
        blip.set_rates(F_CPU as f64, SAMPLE_RATE as f64).unwrap();

        let (producer, consumer) = HeapRb::new(RING_CAPACITY).split();

        (
            APUSource {
                sample_buffer: consumer,
                last: 0,
            },
            APUSink {
                producer,
                last: 0,
                blip,
                // Starting the estimate at the target keeps the first seconds of
                // playback from being retuned against a buffer that is only
                // empty because nothing has been pushed into it yet.
                fill: TARGET_FILL,
            },
        )
    }
}

impl Iterator for APUSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        const SCALE: f32 = 1.0 / -(i16::MIN as f32);

        // Holding the last sample through a gap rather than dropping to zero.
        // The buffer should never run dry -- if it does the emulator is not
        // keeping up and that is the thing to fix -- but a short gap held flat
        // is far less audible than the step a jump to zero puts in the signal.
        self.last = self.sample_buffer.try_pop().unwrap_or(self.last);
        Some(self.last as f32 * SCALE)
    }
}

impl Source for APUSource {
    fn current_span_len(&self) -> Option<usize> {
        return None;
    }

    fn channels(&self) -> rodio::ChannelCount {
        return NonZero::new(1).unwrap();
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        return None;
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        return NonZero::new(SAMPLE_RATE).unwrap();
    }
}

pub struct SoundEngine {
    // Dropping the device sink closes the output stream, so it has to outlive
    // the player rather than being a temporary in `new`.
    _sink: rodio::MixerDeviceSink,
    player: rodio::Player,
}

impl SoundEngine {
    pub fn new() -> Self {
        let sink = Self::open_sink();
        let player = rodio::Player::connect_new(&sink.mixer());

        Self {
            _sink: sink,
            player,
        }
    }

    /// Opens the default output device, asking for a shorter period than rodio
    /// would pick on its own.
    ///
    /// The device empties the ring buffer a whole period at a time, so the ring
    /// has to stay at least a period deep or it runs dry in between and the
    /// gaps are heard as clicks. That makes the period the floor on audio
    /// latency, twice over: once in the device's own buffer and once in the
    /// ring that has to cover it. rodio aims for 50ms, which was arriving as
    /// swings of nearly 100ms here.
    fn open_sink() -> rodio::MixerDeviceSink {
        rodio::DeviceSinkBuilder::from_default_device()
            .and_then(|builder| {
                builder
                    .with_buffer_size(rodio::cpal::BufferSize::Fixed(DEVICE_PERIOD))
                    .open_stream()
            })
            // Not every device will accept a period we chose, and default
            // timing beats no sound at all.
            .or_else(|_| rodio::DeviceSinkBuilder::open_default_sink())
            .expect("open default audio stream")
    }

    pub fn add_source<T>(&mut self, source: T)
    where
        T: Send + rodio::Source + 'static,
    {
        self.player.append(source);
    }
}
