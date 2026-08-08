use crate::nes::apu::{LENGTH_COUNTER_TABLE, envelope::Envelope};

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

    sweep_enabled: bool,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,

    timer_max: u16,
    timer: u16,
    length_counter: u8,

    sequencer_step: u8,

    envelope: Envelope,
}

impl PulseChannel {
    pub fn new() -> Self {
        PulseChannel {
            duty: 0,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            timer_max: 0,
            timer: 0,
            length_counter: 0,
            sequencer_step: 0,
            envelope: Envelope::new(),
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
                self.sweep_enabled = (data & 0x80) != 0;
                self.sweep_period = (data & 0x70) >> 4;
                self.sweep_negate = (data & 0x08) != 0;
                self.sweep_shift = data & 0x07;
            }
            2 => self.timer_max = (self.timer_max & 0xFF00) | (data as u16),
            3 => {
                self.timer_max = (((data & 0x07) as u16) << 8) | (self.timer_max & 0x00FF);
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

        // Clock the sequencer on the transition from 0 to timer_max
        // timer goes t, t-1, ..., 0, t, where t = self.timer_max
        if self.timer == 0 {
            self.sequencer_step = (self.sequencer_step + 1) % 8;
            self.timer = self.timer_max;
        } else {
            self.timer -= 1;
        }
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick();
    }

    pub fn tick_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope.loop_flag() {
            self.length_counter -= 1;
        }
    }

    fn enabled(&self) -> bool {
        self.length_counter > 0 && self.timer_max >= 8
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
