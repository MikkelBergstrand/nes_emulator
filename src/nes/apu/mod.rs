mod adressing;
mod channel;
mod envelope;
mod length_counter;
pub mod mixer;
mod pulse_channel;
mod sweeper;
mod triangle_channel;

use pulse_channel::PulseChannel;

use crate::nes::apu::{mixer::APUMixer, triangle_channel::TriangleChannel};

const ENVELOPE_AND_LINEARCOUNTER: u8 = 0b10;
const LENGTHCOUNTER_AND_SWEEP: u8 = 0b01;

// The frame counter steps at ~240 Hz, which is every 3728.5 APU cycles. The
// half cycle only shifts where the IRQ lands inside a step, so rounding it
// away costs one APU cycle per sequence and nothing audible.
const FRAME_COUNTER_PERIOD: u16 = 3729;

// Defines when to tick what parts of the APU depending on the frame counter mode,
// which is either a 4 or 5-long repeating sequence of rules, as defined below.
const FRAME_COUNTER_RULES: &[&[u8]] = &[
    &[
        // Frame counter mode 0 (bit 7 of status flag)
        ENVELOPE_AND_LINEARCOUNTER,
        ENVELOPE_AND_LINEARCOUNTER | LENGTHCOUNTER_AND_SWEEP,
        ENVELOPE_AND_LINEARCOUNTER,
        ENVELOPE_AND_LINEARCOUNTER | LENGTHCOUNTER_AND_SWEEP,
    ],
    &[
        // Frame counter mode 1 (bit 7 of status flag)
        ENVELOPE_AND_LINEARCOUNTER,
        ENVELOPE_AND_LINEARCOUNTER | LENGTHCOUNTER_AND_SWEEP,
        ENVELOPE_AND_LINEARCOUNTER,
        0,
        ENVELOPE_AND_LINEARCOUNTER | LENGTHCOUNTER_AND_SWEEP,
    ],
];

pub struct APUChannels {
    pub pulse_1: PulseChannel,
    pub pulse_2: PulseChannel,
    pub triangle: TriangleChannel,
}

impl APUChannels {
    pub fn new() -> Self {
        return Self {
            pulse_1: PulseChannel::new(),
            pulse_2: PulseChannel::new(),
            triangle: TriangleChannel::new(),
        };
    }
}

pub struct APU<T: APUMixer> {
    channels: APUChannels,
    mixer: T,
    status: u8,
    frame_counter_mode: u8,
    frame_counter: usize,
    frame_divider: u16,
}

impl<T: APUMixer> APU<T> {
    pub fn new() -> Self {
        APU::<T> {
            channels: APUChannels::new(),
            status: 0,
            frame_counter_mode: 0,
            frame_counter: 0,
            frame_divider: 0,
            mixer: T::new(),
        }
    }

    fn tick_length_counter_and_sweep(&mut self) {
        self.channels.pulse_1.tick_length_counter();
        self.channels.pulse_1.tick_sweep();

        self.channels.pulse_2.tick_length_counter();
        self.channels.pulse_2.tick_sweep();

        self.channels.triangle.tick_length_counter();
    }
    fn tick_envelope_and_linear_counter(&mut self) {
        self.channels.pulse_1.tick_envelope();
        self.channels.pulse_2.tick_envelope();

        self.channels.triangle.tick_linear_counter();
    }

    /// Advances the frame counter by one step and applies that step's rule.
    /// This is the ~240 Hz clock the note lengths and envelopes run on, *not*
    /// the APU cycle clock — see `tick`.
    fn tick_frame_counter(&mut self) {
        let rules = FRAME_COUNTER_RULES[self.frame_counter_mode as usize];
        let step = rules[self.frame_counter];

        if step & ENVELOPE_AND_LINEARCOUNTER != 0 {
            self.tick_envelope_and_linear_counter();
        }
        if step & LENGTHCOUNTER_AND_SWEEP != 0 {
            self.tick_length_counter_and_sweep();
        }

        self.frame_counter = (self.frame_counter + 1) % rules.len();
    }

    /// Advances the APU by one APU cycle, i.e. every second CPU cycle.
    pub fn tick_half_clock(&mut self) {
        self.frame_divider += 1;
        if self.frame_divider >= FRAME_COUNTER_PERIOD {
            self.frame_divider = 0;
            self.tick_frame_counter();
        }

        self.channels.pulse_1.tick();
        self.channels.pulse_2.tick();
    }

    /// Ticks once per CPU clock
    pub fn tick_full_clock(&mut self) {
        self.channels.triangle.tick();
    }

    /// Handles a write to $4017. Resets the sequence, and in mode 1 clocks the
    /// first step immediately rather than a divider period later.
    fn set_frame_counter(&mut self, data: u8) {
        self.frame_counter_mode = (data & 0x80) >> 7;
        self.frame_counter = 0;
        self.frame_divider = 0;

        if self.frame_counter_mode == 1 {
            self.tick_envelope_and_linear_counter();
            self.tick_length_counter_and_sweep();
        }
    }

    pub fn get_output(&self) -> f32 {
        self.mixer.mix(&self.channels)
    }
}
