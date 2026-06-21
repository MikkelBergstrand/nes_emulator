use std::usize;

pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn new(data: &[u8]) -> Self {
        ROM { data: data.to_vec() } 
    }

    pub fn read(&self, addr: u16) -> u8 { 
        self.data[addr as usize] 
    }
}
