pub struct Sweeper {
    pub enabled: bool,
    pub period: u8,
    pub negate: bool,
    pub shift: u8,

    divider: u8,
    reload: bool,
}

impl Sweeper {
    pub fn mute(&self, period: u16) -> bool {
        return period < 8 || self.target_period(period) > 0x7FF;
    }
    pub fn new() -> Self {
        Self {
            enabled: false,
            period: 0,
            negate: false,
            shift: 0,
            reload: false,
            divider: 0,
        }
    }

    pub fn reset(&mut self) {
        self.reload = true;
    }

    // A swepper iteration may alter the channel timer period, hence the
    // function takes in the current period and returns a (possibly altereded) new period value
    pub fn tick(&mut self, period: u16) -> u16 {
        let mut period = period;

        if self.divider == 0 && self.enabled && self.shift > 0 {
            if !self.mute(period) {
                period = self.target_period(period);
            }
        }

        if self.divider == 0 || self.reload {
            self.divider = self.period;
            self.reload = false;
        } else {
            self.divider -= 1;
        }

        period
    }

    pub fn target_period(&self, period: u16) -> u16 {
        let change = (period >> self.shift) as i16;
        let change = if self.negate { -change - 1 } else { change };

        (period as i16 + change).max(0) as u16
    }
}
