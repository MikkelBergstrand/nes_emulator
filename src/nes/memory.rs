use core::panic;

use super::NES;

#[derive(Debug)]
enum Target {
    RAM(u16), PPU(u8), ROM(u16), Unspecified
}

impl NES {
    pub fn write_addr(&mut self, addr: u16, value: u8) {

        let target = self.resolve_mmap(addr);
        println!("Writing to {:04x}, {:?} {:02x}", addr, target, value);
        match target {
            Target::RAM(addr) => { self.ram[addr] = value; }
            Target::PPU(addr) => { self.ppu.write(addr, value); }
            Target::ROM(_) => { println!("Attempted write to ROM"); }
            Target::Unspecified => {}
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        
        let target = self.resolve_mmap(addr);

        let ret = match target {
            Target::RAM(addr) => { self.ram[addr] }
            Target::PPU(addr) => { self.ppu.read(addr as u8) }
            Target::ROM(addr) => { self.rom.read(addr) }
            Target::Unspecified => { 0 }
        };

        println!("Reading from {:04x}, {:?} = {:02x}", addr, target, ret);
        ret
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        // Note little-endian format!
        let low = self.read(addr) as u16;
        let high = self.read(addr.wrapping_add(1)) as u16;
        return (high << 8) | low;
    }

    // Takes in a general 16-bit address, and decides which component is addressed.
    fn resolve_mmap(&mut self, addr: u16) -> Target {
        match addr {
            0x0000..0x2000  => Target::RAM(addr & 0x0FFF),
            0x2000..0x4000  => Target::PPU((addr as u8) % 8),
            0x8000..=0xFFFF => Target::ROM(addr & 0x7FFF),
            _ => Target::Unspecified
        }
    }

}
