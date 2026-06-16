
pub struct PPU {
    value: u8
}

impl PPU {
    pub fn new() -> Self {
        Self{
            value: 0
        }
    }

    pub fn write(&self, addr: u8) {

    }

    pub fn read(&mut self, addr: u8) -> u8 {
        return self.value;
    }
}
