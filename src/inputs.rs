use std::usize;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct InputFlag: u8 {
        const A    = 1 << 0;
        const B     = 1 << 1;
        const SELECT      = 1 << 2;
        const START  = 1 << 3;
        const UP    = 1 << 4;
        const DOWN = 1 << 5;
        const LEFT = 1 << 6;
        const RIGHT = 1 << 7;
    }
}

pub struct Inputs {
    inputs: InputFlag,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            inputs: InputFlag::empty(),
        }
    }

    pub fn set(&mut self, t: InputFlag, v: bool) {
        self.inputs.set(t, v);
    }
    
    pub fn get(&mut self, t: InputFlag) -> bool {
        self.inputs.contains(t)
    }

    pub fn get_input_byte(&self) -> InputFlag  {
        self.inputs
    }
    
}
