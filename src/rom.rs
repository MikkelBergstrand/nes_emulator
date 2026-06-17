use std::usize;

pub struct ROM {
    data: [u8; 1 << 15],
}

impl ROM {
    pub fn new(data: &[u8]) -> Self {
        ROM { data: data.try_into().unwrap() } 
    }

    pub fn read(&self, addr: u8) -> u8 { self.data[addr as usize] }
}
