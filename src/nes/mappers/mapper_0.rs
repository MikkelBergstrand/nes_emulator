use std::usize;

use crate::nes::{mappers::Mapper, nes_parser::NESData};
use crate::nes::nes_parser::NametableArrangement;


pub struct NROM {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    nametable: NametableArrangement,
    n_prg_banks: u8,
}

impl NROM {
    pub fn new(data: NESData) -> Self {
        Self {
            prg_rom: data.prg_rom,
            chr_rom: data.chr_rom,
            n_prg_banks: data.header.prg_rom_size as u8,
            nametable: data.header.nametable_arrangement,
        }
    }
}

impl Mapper for NROM {
    fn ppu_read(&mut self, addr: u16) -> u8 {
        return self.chr_rom[(addr & 0x1FFF) as usize]
    }

    fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x8000..=0xFFFF => self.prg_rom[(addr & if  self.n_prg_banks > 1 { 0x7FFF } else { 0x3FFF }) as usize],
            _ =>  0
        }
    }

    fn nametable_arrangement(&self) -> NametableArrangement {
        self.nametable
    }
}
