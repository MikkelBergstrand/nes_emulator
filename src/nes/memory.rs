use super::NES;

#[derive(Debug)]
enum Target {
    RAM(u16),
    PPU(u8),
    APU(u16),
    Mapper(u16),
    Controller,
    Unspecified,
    OAMDMA,
}

impl NES {
    pub fn write_addr(&mut self, addr: u16, value: u8) {
        let target = self.resolve_mmap(addr);
        match target {
            Target::APU(addr) => {
                self.apu.write(addr, value);
            }
            Target::RAM(addr) => {
                self.ram[addr] = value;
            }
            Target::PPU(addr) => {
                self.ppu.write(&mut self.mapper, addr, value);
            }
            Target::Mapper(addr) => {
                self.mapper.cpu_write(addr, value);
            }
            Target::Controller => {
                self.input_controller.write(value);
            }
            Target::OAMDMA => {
                self.oam_dma(value);
            }
            Target::Unspecified => {}
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let target = self.resolve_mmap(addr);

        let ret = match target {
            Target::APU(addr) => self.apu.read(addr),
            Target::RAM(addr) => self.ram[addr],
            Target::PPU(addr) => self.ppu.read(&mut self.mapper, addr as u8),
            Target::Mapper(addr) => self.mapper.cpu_read(addr),
            Target::Controller => self.input_controller.read(0),
            Target::OAMDMA => 0,
            Target::Unspecified => 0,
        };

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
            0x0000..0x2000 => Target::RAM(addr & 0x07FF),
            0x2000..0x4000 => Target::PPU((addr as u8) % 8),
            0x4000..=0x4013 | 0x4015 | 0x4017 => Target::APU(addr),
            0x4016 => Target::Controller,
            0x4014 => Target::OAMDMA,
            0x4020..=0xFFFF => Target::Mapper(addr),
            _ => Target::Unspecified,
        }
    }
}
