pub struct InputController {
    strobe: bool,
    inputs: u8,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            strobe: false,
            inputs: 0,
        }
    }

    pub fn write(&mut self, val: u8) {
        self.strobe = (val & 1) != 0;
    }    

    pub fn set_controller_state(&mut self, state: u8) {
        self.inputs = state;
    }

    pub fn read(&mut self, controller: u8) -> u8 {
        let ret = self.inputs & 1;
        if !self.strobe {
            self.inputs >>= 1;
        }
        ret
    }

}
