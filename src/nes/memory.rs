use core::panic;

use super::NES;

enum Target {
    CPU(u16), PPU(u8), ROM(u16)
}

impl NES {
    pub fn write_addr(&mut self, addr: u16, value: u8) {
        let target = self.resolve_mmap(addr);
        match target {
            Target::CPU(addr) => { self.ram[addr] = value; }
            Target::PPU(addr) => { self.ppu.write(addr); }
            Target::ROM(_) => { println!("Attempted write to ROM"); }
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let target = self.resolve_mmap(addr);
        match target {
            Target::CPU(addr) => { self.ram[addr] }
            Target::PPU(addr) => { self.ppu.read(addr as u8) }
            Target::ROM(addr) => { self.rom.read(addr as u8) }
        }
    }

    // Takes in a general 16-bit address, and decides which component is addressed.
    fn resolve_mmap(&mut self, addr: u16) -> Target {
        match addr {
            0x0000..0x2000  => Target::CPU(addr & 0x01FF),
            0x2000..0x4000  => Target::CPU(addr % 8),
            0x8000..=0xFFFF => Target::ROM(addr & 0x01FF),
            _ => panic!("Unimplemented")
        }
    }
}
