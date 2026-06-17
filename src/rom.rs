use std::usize;

pub struct ROM {
    data: [u8; 1 << 15],
}

impl ROM {
    pub fn new(data: &[u8]) -> Self {
        ROM { data: data.try_into().unwrap() } 
    }

    pub fn read(&self, addr: u16) -> u8 { 
        println!("ROM read: {:x}", addr);
        self.data[addr as usize] 
    }
}
