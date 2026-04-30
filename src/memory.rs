use std::ops::{Index, IndexMut};

pub struct Memory {
    data: [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        Self {
            data: [0; 65536],
        }
    } 

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
}

impl Index<u16> for Memory {
    type Output = u8;
    
    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }

}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }

}
