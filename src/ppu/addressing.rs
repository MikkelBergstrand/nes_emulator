use std::usize;

use crate::nes_parser::NametableArrangement;

pub struct PPUMemoryMap {
    chr_data: Vec<u8>, // Pattern table data
    nametable_arrangement: NametableArrangement,
    vram: [u8; 2048],
    pallette_ram: [u8; 32],   
}


impl PPUMemoryMap {
    pub fn new(chr_data: &[u8], nametable_arrangement: NametableArrangement) -> Self {
        PPUMemoryMap{
           chr_data: chr_data.to_vec(),
           nametable_arrangement,
           vram: [0u8; 2048],
           pallette_ram: [0u8; 32],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let addr = addr & 0x3FFF;
        //println!("Writing to VRAM {:04X}", addr);
        match addr {
            0x0000..0x2000 => { println!("Writing to ROM"); },
            0x2000..=0x2FFF => { self.vram[self.nametable_addr(addr) as usize] = data; },
            // Address space typically(?) mirrors the above address space
            0x3000..=0x3EFF => { self.vram[self.nametable_addr(addr & !(1 << 12)) as usize] = data; },
            0x3F00..=0x3FFF => { self.pallette_ram[(addr & 0x001F) as usize] = data; },
            _ => ()
        };

        //println!("Write {:02X} to {:04X}", data, addr);
    }

    pub fn read(&self, addr: u16) -> u8 {
        let ret = match addr {
            0x0000..0x2000 => self.chr_data[addr as usize],
            0x2000..=0x2FFF => self.vram[self.nametable_addr(addr) as usize],
            // Address space typically(?) mirrors the above address space
            0x3000..=0x3EFF => self.vram[self.nametable_addr(addr & !(1 << 12)) as usize],
            0x3F00..=0x3FFF => self.pallette_ram[(addr & 0x001F) as usize],
            _ => { panic!("Bad read"); }
        };
        //println!("Read {:02X} from {:04X}", ret, addr);
        ret
    }

    fn nametable_addr(&self, addr: u16) -> u16 {
        // Arrangement dictate which nametable segments are mirrored 
        // Mirror by effectively forcing the "irrelevant bit" to 0
        let ret_addr = match self.nametable_arrangement {
            NametableArrangement::Vertical => { addr & !(1 << 10) }
            NametableArrangement::Horizontal => { addr & !(1 << 11) }
        };

        ret_addr - 0x2000
    }

}
