use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::{HeapCons, HeapProd, HeapRb};
use rodio::Source;
use std::num::NonZero;

use crate::nes::F_CPU;

const SAMPLE_RATE: u32 = 48_000;
const BLIP_CAPACITY: u32 = SAMPLE_RATE / 10;

pub struct APUSink {
    producer: HeapProd<i16>,
    blip: blip_buf::BlipBuf,
    last: i32,
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

    pub fn vacant_len(&self) -> usize {
        self.producer.vacant_len()
    }

    /// How many CPU clocks of emulation the ring buffer's free space can
    /// absorb. Accounts for blip's sub-sample phase, so gating on this has no
    /// rounding slop against a real frame length.
    pub fn clocks_free(&self) -> u32 {
        let headroom = (BLIP_CAPACITY as usize).saturating_sub(self.blip.samples_avail() as usize);
        self.blip
            .clocks_needed(self.vacant_len().min(headroom) as u32)
            .unwrap_or(0)
    }
}

pub struct APUSource {
    sample_buffer: HeapCons<i16>,
}

impl APUSource {
    pub fn new() -> (Self, APUSink) {
        let mut blip = blip_buf::BlipBuf::new(BLIP_CAPACITY);
        blip.set_rates(F_CPU as f64, SAMPLE_RATE as f64).unwrap();

        let (producer, consumer) = HeapRb::new(SAMPLE_RATE as usize * 80 / 1000).split();

        (
            APUSource {
                sample_buffer: consumer,
            },
            APUSink {
                producer,
                last: 0,
                blip,
            },
        )
    }
}

impl Iterator for APUSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        const SCALE: f32 = 1.0 / -(i16::MIN as f32);
        Some(self.sample_buffer.try_pop().unwrap_or(0) as f32 * SCALE)
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
        let sink =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(&sink.mixer());

        Self {
            _sink: sink,
            player,
        }
    }

    pub fn add_source<T>(&mut self, source: T)
    where
        T: Send + rodio::Source + 'static,
    {
        self.player.append(source);
    }
}
