use std::ops::{Index, IndexMut};

pub struct RAM {
    data: [u8; 2048] 
}

impl RAM {
    pub fn new() -> RAM {
        Self {
            data: [0; 2048],
        }
    } 
}

impl Index<u16> for RAM {
    type Output = u8;
    
    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }

}

impl IndexMut<u16> for RAM {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }

}
