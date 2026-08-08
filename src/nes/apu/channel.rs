pub struct PulseChannel {
    duty: u8,
    envelope_loop: bool,
    constant_vol: bool,
    volume: u8,

    sweep_enabled: bool,
    sweep_period: u8,
    sweep_negate: bool,
    sweep_shift: u8,

    timer_max: u16,
    timer: u16,
    length_counter: u8,
}

impl PulseChannel {
    pub fn new() -> Self {
        PulseChannel {
            duty: 0,
            envelope_loop: false,
            constant_vol: false,
            volume: 0,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            timer_max: 0,
            timer: 0,
            length_counter: 0,
        }
    }

    pub fn set(&mut self, reg: u8, data: u8) {
        match reg {
            0 => {
                self.duty = (data & 0xC0) >> 6;
                self.envelope_loop = (data & 0x20) != 0;
                self.constant_vol = (data & 0x10) != 0;
                self.volume = data & 0xF;
            }
            1 => {
                self.sweep_enabled = (data & 0x80) != 0;
                self.sweep_period = (data & 0x70) >> 4;
                self.sweep_negate = (data & 0x08) != 0;
                self.sweep_shift = data & 0x07;
            }
            2 => self.timer_max = (self.timer_max & 0xF0) | (data as u16),
            3 => {
                self.timer_max = (((data & 0x07) as u16) << 8) | ((self.timer_max as u16) & 0x0F);
                self.length_counter = data & 0xF8;
            }
            _ => panic!("PulseChannel invalid address"),
        }
    }

    pub fn tick(&mut self, enabled: bool) {
        if !enabled {
            self.length_counter = 0;
            return;
        }

        self.timer = (self.timer.wrapping_sub(1)) % (self.timer_max + 1);

        if self.length_counter > 0 && !self.envelope_loop {
            self.length_counter -= 1;
        }
    }

    fn enabled(&self) -> bool {
        self.length_counter > 0 && self.timer_max >= 8
    }

    fn duty_cycle(&self) -> f32 {
        match self.duty {
            0 => 0.125,
            1 => 0.25,
            2 => 0.5,
            3 => 0.75,
            _ => panic!("Bad duty cycle param"),
        }
    }

    pub fn get_output(&self) -> f32 {
        if self.enabled() { 1.0 } else { 0.0 }
    }
}
