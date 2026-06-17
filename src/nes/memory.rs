use core::panic;

use super::NES;

#[derive(Debug)]
enum Target {
    RAM(u16), PPU(u8), ROM(u16)
}

impl NES {
    pub fn write_addr(&mut self, addr: u16, value: u8) {

        let target = self.resolve_mmap(addr);
        println!("Writing to {:x}, {:?}", addr, target);
        match target {
            Target::RAM(addr) => { self.ram[addr] = value; }
            Target::PPU(addr) => { self.ppu.write(addr); }
            Target::ROM(_) => { println!("Attempted write to ROM"); }
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        
        let target = self.resolve_mmap(addr);

        println!("Reading from {:x}, {:?}", addr, target);
        match target {
            Target::RAM(addr) => { self.ram[addr] }
            Target::PPU(addr) => { self.ppu.read(addr as u8) }
            Target::ROM(addr) => { self.rom.read(addr) }
        }
    }

    // Takes in a general 16-bit address, and decides which component is addressed.
    fn resolve_mmap(&mut self, addr: u16) -> Target {
        match addr {
            0x0000..0x2000  => Target::RAM(addr & 0x01FF),
            0x2000..0x4000  => Target::RAM(addr % 8),
            0x8000..=0xFFFF => Target::ROM(addr & 0x01FF),
            _ => panic!("Unimplemented")
        }
    }
}
