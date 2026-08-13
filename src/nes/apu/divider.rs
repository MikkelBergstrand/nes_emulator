pub struct Divider {
    current: u16,
    target: u16,
}

impl Divider {
    pub fn new(target: u16) -> Self {
        Divider { current: 0, target }
    }

    // Tick the divider. Returns whether or not the target has been
    // reached, and a period has elapsed.
    pub fn tick(&mut self) -> bool {
        return if self.current == 0 {
            self.current = self.target;
            true
        } else {
            self.current -= 1;
            false
        };
    }

    pub fn set_period(&mut self, target: u16) {
        self.target = target
    }

    pub fn get_period(&self) -> u16 {
        self.target
    }
}
