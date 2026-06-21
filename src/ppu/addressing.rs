use std::usize;

use crate::nes_parser::NametableArrangement;

pub struct PPUMemoryMap {
    chr_data: Vec<u8>, // Pattern table data
    chr_bank: usize,   // Selected 8KB CHR bank (CNROM / mapper 3)
    nametable_arrangement: NametableArrangement,
    vram: [u8; 2048],
    pallette_ram: [u8; 32],
}


// Map a $3F00-$3FFF address to a palette-RAM index (0-0x1F), folding the
// hardware mirrors: $3F10/$3F14/$3F18/$3F1C alias $3F00/$3F04/$3F08/$3F0C.
// SMB1 sets the sky backdrop by writing it to $3F10, relying on this mirror.
fn palette_index(addr: u16) -> usize {
    let mut i = (addr & 0x001F) as usize;
    if i & 0x13 == 0x10 {
        i -= 0x10;
    }
    i
}

impl PPUMemoryMap {
    pub fn new(chr_data: &[u8], nametable_arrangement: NametableArrangement) -> Self {
        PPUMemoryMap{
           chr_data: chr_data.to_vec(),
           chr_bank: 0,
           nametable_arrangement,
           vram: [0u8; 2048],
           pallette_ram: [0u8; 32],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let addr = addr & 0x3FFF;
        //println!("Writing to VRAM {:04X}", addr);
        match addr {
            0x0000..0x2000 => (),
            0x2000..=0x2FFF => { self.vram[self.nametable_addr(addr) as usize] = data; },
            // Address space typically(?) mirrors the above address space
            0x3000..=0x3EFF => { self.vram[self.nametable_addr(addr & !(1 << 12)) as usize] = data; },
            0x3F00..=0x3FFF => { self.pallette_ram[palette_index(addr)] = data; },
            _ => ()
        };
    }

    pub fn read(&self, addr: u16) -> u8 {
        let ret = match addr {
            0x0000..0x2000 => self.chr_data[addr as usize],
            0x2000..=0x2FFF => self.vram[self.nametable_addr(addr) as usize],
            // Address space typically(?) mirrors the above address space
            0x3000..=0x3EFF => self.vram[self.nametable_addr(addr) as usize],
            0x3F00..=0x3FFF => self.pallette_ram[palette_index(addr)],
            _ => { panic!("Bad read"); }
        };
        //println!("Read {:02X} from {:04X}", ret, addr);
        ret
    }

    fn nametable_addr(&self, addr: u16) -> u16 {
        let table = (addr >> 10) & 3;
        let offset = addr & 0x03FF;

        let physical = match self.nametable_arrangement {
            NametableArrangement::Vertical => { (table >> 1) & 1 }
            NametableArrangement::Horizontal => { table & 1 }
        };

        physical * 0x0400 + offset
    }

}
