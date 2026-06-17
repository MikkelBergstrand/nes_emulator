
pub struct PPU {
    data: [u8; 8],
}

impl PPU {
    pub fn new() -> Self {
        Self{
            data: [0; 8]
        }
    }

    pub fn write(&self, addr: u8) {

    }

    pub fn read(&mut self, addr: u8) -> u8 {
        return self.data[addr as usize];
    }
}
