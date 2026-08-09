use crate::nes::{F_CPU, apu::channel::PulseChannel};

mod adressing;
pub mod channel;
mod envelope;
mod sweeper;

pub const F_APU: f32 = F_CPU / 2.0;

#[rustfmt::skip]
pub const LENGTH_COUNTER_TABLE: [u8; 0x20] = [
    // even index: note length            odd index: linear ramp
    10,  254, // 0x00 sixteenth           0x01
    20,  2,   // 0x02 eighth              0x03
    40,  4,   // 0x04 quarter             0x05
    80,  6,   // 0x06 half                0x07
    160, 8,   // 0x08 whole               0x09
    60,  10,  // 0x0A dotted quarter      0x0B
    14,  12,  // 0x0C eighth triplet      0x0D
    26,  14,  // 0x0E quarter triplet     0x0F
    12,  16,  // 0x10 sixteenth           0x11
    24,  18,  // 0x12 eighth              0x13
    48,  20,  // 0x14 quarter             0x15
    96,  22,  // 0x16 half                0x17
    192, 24,  // 0x18 whole               0x19
    72,  26,  // 0x1A dotted quarter      0x1B
    16,  28,  // 0x1C eighth triplet      0x1D
    32,  30,  // 0x1E quarter triplet     0x1F
];

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

pub struct APU {
    pub pulse_channel_1: PulseChannel,
    pub pulse_channel_2: PulseChannel,
    status: u8,
    frame_counter_mode: u8,
    frame_counter: usize,
    frame_divider: u16,
}

impl APU {
    pub fn new() -> Self {
        APU {
            pulse_channel_1: PulseChannel::new(),
            pulse_channel_2: PulseChannel::new(),
            status: 0,
            frame_counter_mode: 0,
            frame_counter: 0,
            frame_divider: 0,
        }
    }

    fn tick_length_counter_and_sweep(&mut self) {
        self.pulse_channel_1.tick_length_counter();
        self.pulse_channel_1.tick_sweep();

        self.pulse_channel_2.tick_length_counter();
        self.pulse_channel_2.tick_sweep();
    }
    fn tick_envelope_and_linear_counter(&mut self) {
        self.pulse_channel_1.tick_envelope();
        self.pulse_channel_2.tick_envelope();
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
    pub fn tick(&mut self) {
        self.frame_divider += 1;
        if self.frame_divider >= FRAME_COUNTER_PERIOD {
            self.frame_divider = 0;
            self.tick_frame_counter();
        }

        self.pulse_channel_1.tick(self.status & 1 != 0);
        self.pulse_channel_2.tick(self.status & 2 != 0);
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
        let psum =
            self.pulse_channel_1.get_output() as f32 + self.pulse_channel_2.get_output() as f32;
        return 95.88 / ((8128.0 / psum) + 100.0);
    }
}

pub struct APUStatus {}
