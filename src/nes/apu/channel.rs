use crate::nes::apu::{LENGTH_COUNTER_TABLE, envelope::Envelope, sweeper::Sweeper};

// Defines the output of the sequencer
// First index is by duty cycle mode, which may be set by writing to 0x4000 or 0x4004
// Second index is the current sequencer step.
// The sequencer is clocked every time the timer value (t) wraps around to zero, see tick()
const PULSE_SEQUENCER: &[&[u8]] = &[
    &[0, 1, 0, 0, 0, 0, 0, 0], //12.5%
    &[0, 1, 1, 0, 0, 0, 0, 0], //25%
    &[0, 1, 1, 1, 1, 0, 0, 0], //50%
    &[1, 0, 0, 1, 1, 1, 1, 1], //25% negated
];

pub struct PulseChannel {
    duty: u8,

    t: u16,
    length_counter: u8,

    sequencer_step: u8,

    envelope: Envelope,
    sweeper: Sweeper,
}

impl PulseChannel {
    pub fn new() -> Self {
        PulseChannel {
            duty: 0,
            t: 0,
            length_counter: 0,
            sequencer_step: 0,
            envelope: Envelope::new(),
            sweeper: Sweeper::new(),
        }
    }

    pub fn set(&mut self, reg: u8, data: u8) {
        match reg {
            0 => {
                self.duty = (data & 0xC0) >> 6;
                self.envelope
                    .set_params((data & 0x10) != 0, data & 0xF, data & 0x20 != 0);
            }
            1 => {
                self.sweeper.enabled = (data & 0x80) != 0;
                self.sweeper.period = (data & 0x70) >> 4;
                self.sweeper.negate = (data & 0x08) != 0;
                self.sweeper.shift = data & 0x07;
                self.sweeper.reset();
            }
            2 => self
                .sweeper
                .set_pulse_period((self.sweeper.pulse_period() & 0xFF00) | (data as u16)),
            3 => {
                self.sweeper.set_pulse_period(
                    (((data & 0x07) as u16) << 8) | (self.sweeper.pulse_period() & 0x00FF),
                );
                self.length_counter = LENGTH_COUNTER_TABLE[((data & 0xF8) >> 3) as usize];
                self.envelope.set_start();

                // Side effects of write
                self.sequencer_step = 0;
            }
            _ => panic!("PulseChannel invalid address"),
        }
    }

    pub fn tick(&mut self, enabled: bool) {
        if !enabled {
            self.length_counter = 0;
            return;
        }

        // Clock the sequencer on the transition from 0 to period
        if self.t == 0 {
            self.sequencer_step = (self.sequencer_step + 1) % 8;
            self.t = self.sweeper.pulse_period();
        } else {
            self.t -= 1;
        }
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick();
    }

    pub fn tick_sweep(&mut self) {
        self.sweeper.tick();
    }

    pub fn tick_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope.loop_flag() {
            self.length_counter -= 1;
        }
    }

    fn enabled(&self) -> bool {
        self.length_counter > 0 && !self.sweeper.mute()
    }

    pub fn get_output(&self) -> u16 {
        if self.enabled() {
            // PULSE_SEQUENCER contains the  current duty mode (0-3) and the
            // current sequencer step, which is an 8-step looping sequence
            // volume is determined by the envelope
            (self.envelope.volume()
                * PULSE_SEQUENCER[self.duty as usize][self.sequencer_step as usize])
                as u16
        } else {
            0
        }
    }
}
