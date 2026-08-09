pub struct Sweeper {
    pub enabled: bool,
    pub period: u8,
    pub negate: bool,
    pub shift: u8,

    divider: u8,
    reload: bool,

    pulse_period: u16,
}

impl Sweeper {
    pub fn mute(&self) -> bool {
        return self.pulse_period < 8 || self.target_period() > 0x7FF;
    }
    pub fn new() -> Self {
        Self {
            enabled: false,
            period: 0,
            negate: false,
            shift: 0,
            reload: false,
            divider: 0,
            pulse_period: 0,
        }
    }

    pub fn reset(&mut self) {
        self.reload = true;
    }

    pub fn set_pulse_period(&mut self, period: u16) {
        self.pulse_period = period;
    }

    pub fn pulse_period(&self) -> u16 {
        self.pulse_period
    }

    pub fn tick(&mut self) {
        if self.divider == 0 && self.enabled && self.shift > 0 {
            let target = self.target_period();
            if !self.mute() {
                self.pulse_period = target;
            }
        }

        if self.divider == 0 || self.reload {
            self.divider = self.period;
            self.reload = false;
        } else {
            self.divider -= 1;
        }
    }

    pub fn target_period(&self) -> u16 {
        let change = (self.pulse_period >> self.shift) as i16;
        let change = if self.negate { -change - 1 } else { change };

        (self.pulse_period as i16 + change).max(0) as u16
    }
}
