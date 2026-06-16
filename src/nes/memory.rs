use super::NES;

enum Target {
    CPU(u16), PPU(u8)
}

impl NES {
    pub fn write_addr(&mut self, addr: u16, value: u8) {
        let target = self.resolve_mmap(addr);
        match target {
            Target::CPU(addr) => { self.ram[addr] = value; }
            Target::PPU(addr) => { self.ppu.write(addr); }
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let target = self.resolve_mmap(addr);
        match target {
            Target::CPU(addr) => { self.ram[addr] }
            Target::PPU(addr) => { self.ppu.read(addr as u8) }
        }
    }

    // Takes in a general 16-bit address, and decides which component is addressed.
    fn resolve_mmap(&mut self, addr: u16) -> Target {

        let high_bits = addr >> 13; 
        let low_bits = addr & 0x01FF;
        if !high_bits & 0b110 == 0b110 { // RAM - High bits are either 000 or 001 (it is mirrored)
            return Target::CPU(low_bits);
        } else if high_bits == 0b001 {
            // PPU is mirrored every 8th byte
            return Target::PPU((low_bits % 8) as u8);
        } else {
            panic!("Unimplemented");
        }
    }
}
