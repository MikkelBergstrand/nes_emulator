pub struct Envelope {
    start: bool,
    constant_volume: bool,
    decay_level: u8,
    loop_flag: bool,
    volume: u8,
    divider: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            start: true,
            decay_level: 15,
            loop_flag: false,
            volume: 0,
            divider: 0,
            constant_volume: false,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume;
    }

    pub fn set_constant_volume_flag(&mut self, value: bool) {
        self.constant_volume = value;
    }

    pub fn set_loop_flag(&mut self, value: bool) {
        self.loop_flag = value;
    }

    pub fn tick(&mut self) {
        if self.start {
            self.start = false;
            self.decay_level = 15;
            self.divider = self.volume;
        } else {
            self.clock_divider();
        }
    }

    // set the start flag. occurs as a side effect of writing to length counter load register.
    pub fn set_start(&mut self) {
        self.start = true;
    }

    fn clock_divider(&mut self) {
        if self.divider == 0 {
            self.divider = self.volume;
            if self.decay_level > 0 {
                self.decay_level -= 1;
            } else if self.loop_flag {
                self.decay_level = 15;
            }
        } else {
            self.divider -= 1;
        }
    }

    pub fn volume(&self) -> u8 {
        if self.constant_volume {
            self.volume
        } else {
            self.decay_level
        }
    }

    pub fn loop_flag(&self) -> bool {
        self.loop_flag
    }
}
