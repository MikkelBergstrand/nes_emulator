use std::usize;

use crate::nes::{mappers::Mapper, nes_parser::NametableArrangement}; 

pub struct PPUMemoryMap {
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
    pub fn new() -> Self {
        PPUMemoryMap{
           vram: [0u8; 2048],
           pallette_ram: [0u8; 32],
        }
    }

    pub fn write(&mut self, mapper: &mut Box<dyn Mapper>, addr: u16, data: u8) {
        let addr = addr & 0x3FFF;
        //println!("Writing to VRAM {:04X}", addr);
        match addr {
            0x0000..0x2000 =>  { mapper.ppu_write(addr, data); },
            0x2000..=0x2FFF => { self.vram[nametable_addr(mapper.nametable_arrangement(), addr) as usize] = data; },
            // Address space typically(?) mirrors the above address space
            0x3000..=0x3EFF => { self.vram[nametable_addr(mapper.nametable_arrangement(), addr & !(1 << 12)) as usize] = data; },
            0x3F00..=0x3FFF => { self.pallette_ram[palette_index(addr)] = data; },
            _ => ()
        };
    }

    pub fn read(&self, mapper: &mut Box<dyn Mapper>, addr: u16) -> u8 {
        let ret = match addr {
            0x0000..0x2000 => mapper.ppu_read(addr), 
            0x2000..=0x2FFF => self.vram[nametable_addr(mapper.nametable_arrangement(), addr) as usize],
            // Address space typically(?) mirrors the above address space
            0x3000..=0x3EFF => self.vram[nametable_addr(mapper.nametable_arrangement(), addr) as usize],
            0x3F00..=0x3FFF => self.pallette_ram[palette_index(addr)],
            _ => { panic!("Bad read"); }
        };
        //println!("Read {:02X} from {:04X}", ret, addr);
        ret
    }
}

fn nametable_addr(nametable_arrangement: NametableArrangement, addr: u16) -> u16 {
    // Nametable are arranged by 2x2. Starting at address 0x2000 and ending at
    // address 0x2FFF. So the start of each table is at 0x2000, 0x2400, 0x2800 0x2C00
 
    // Table index 0 - 3 is  determined byA11, A10
    let table = (addr >> 10) & 3;

    // Offset A0-A9
    let offset = addr & 0x03FF;

    let physical = match nametable_arrangement {
        // Horizontal mirroring: 0x2400 -> 0x2000, 0x2C00 -> 0x2800
        NametableArrangement::Vertical => (table & 2) >> 1,
        // Vertical mirroring 0x2800 -> 0x2000, 0x2C00 -> 0x2400
        NametableArrangement::Horizontal => table & 1,
        // Single screen
        NametableArrangement::OneScreen(0) => 0,
        NametableArrangement::OneScreen(1) => 1,
        _ => panic!("Invalid nametable config")
    };
    

    physical * 0x0400 + offset
}
