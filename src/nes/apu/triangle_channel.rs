use crate::nes::apu::channel::{timer_high, timer_low};

use super::length_counter::LengthCounter;

const SEQUENCE: &[u8] = &[
    15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
    13, 14, 15,
];
pub struct TriangleChannel {
    pub length_counter: LengthCounter,

    period: u16,
    t: u16,
    sequence_idx: u8,

    linear_counter: u8,
    linear_counter_reload_value: u8,
    linear_counter_reload: bool,
    control: bool,

    enabled: bool,
}

impl TriangleChannel {
    pub fn new() -> Self {
        Self {
            t: 0,
            sequence_idx: 0,
            linear_counter: 0,
            length_counter: LengthCounter::new(),
            period: 0,
            enabled: false,
            linear_counter_reload: false,
            linear_counter_reload_value: 0,
            control: false,
        }
    }

    pub fn set(&mut self, reg: u8, data: u8) {
        match reg {
            0 => {
                self.length_counter.set_halt(data & 0x80 != 0);
                self.control = data & 0x80 != 0;
                self.linear_counter_reload_value = data & 0x7F;
            }
            1 => (),
            2 => {
                self.period = timer_low!(self.period, data);
            }
            3 => {
                self.period = timer_high!(self.period, data);
                self.length_counter.set_counter((data & 0xF8) >> 3);
                self.linear_counter_reload = true;
            }
            _ => panic!("Impossible"),
        };
    }

    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
        self.length_counter.set_enabled(v);
    }

    fn enabled(&self) -> bool {
        self.enabled && self.length_counter.active()
    }

    pub fn tick_length_counter(&mut self) {
        self.length_counter.tick();
    }

    pub fn tick_linear_counter(&mut self) {
        if self.linear_counter_reload {
            self.linear_counter = self.linear_counter_reload_value;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }

        if !self.control {
            self.linear_counter_reload = false;
        }
    }

    pub fn tick(&mut self) {
        if self.t == 0 {
            if self.linear_counter > 0 && self.length_counter.active() {
                self.sequence_idx = (self.sequence_idx + 1) % SEQUENCE.len() as u8;
            }
            self.t = self.period;
        } else {
            self.t -= 1;
        }
    }

    pub fn get_output(&self) -> u16 {
        if self.enabled() {
            SEQUENCE[self.sequence_idx as usize] as u16
        } else {
            0
        }
    }
}
