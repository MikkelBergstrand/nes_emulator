use super::{divider::Divider, envelope::Envelope, length_counter::LengthCounter};
use std::usize;

const TIMER_PERIOD_TABLE: &[u16] = &[
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

pub struct NoiseChannel {
    divider: Divider,
    length_counter: LengthCounter,
    envelope: Envelope,
    mode: bool,
    shift_reg: u16,
    enabled: bool,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            divider: Divider::new(0),
            length_counter: LengthCounter::new(),
            envelope: Envelope::new(),
            mode: false,
            shift_reg: 1,
            enabled: false,
        }
    }

    pub fn set(&mut self, reg: u8, data: u8) {
        match reg {
            0 => {
                self.envelope.set_constant_volume_flag(data & 0x10 != 0);
                self.envelope.set_volume(data & 0x0F);
                self.length_counter.set_halt(data & 0x20 != 0);
            }
            1 => (),
            2 => {
                self.mode = data & 0x80 != 0;
                self.divider
                    .set_period(TIMER_PERIOD_TABLE[(data & 0x0F) as usize] / 2);
            }
            3 => {
                self.length_counter.set_counter((data & 0xF8) >> 3);
                self.envelope.set_start();
            }
            _ => panic!("Unimplemented"),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.length_counter.set_enabled(self.enabled);
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick();
    }
    pub fn tick_length_counter(&mut self) {
        self.length_counter.tick();
    }

    // Clocked every other CPU cycle.
    pub fn tick(&mut self) {
        if self.divider.tick() {
            // Feedback is XOR of shift bit 1 and either shift bit 6 or 1, depending on mode flag
            let feedback = (self.shift_reg & 1)
                ^ if self.mode {
                    (self.shift_reg >> 6) & 1
                } else {
                    (self.shift_reg >> 1) & 1
                };
            // Shift the register right, and load new value in the 14th bit
            // The result is a pseudo-random sequence.
            self.shift_reg >>= 1;
            self.shift_reg = (self.shift_reg & !(1 << 14)) | (feedback << 14);
        }
    }

    pub fn get_output(&self) -> u16 {
        if self.enabled && self.length_counter.active() && (self.shift_reg & 1 != 0) {
            self.envelope.volume() as u16
        } else {
            0
        }
    }
}
