use std::usize;

use crate::nes::{mappers::Mapper, nes_parser::{NESData, NametableArrangement}};

pub struct MMC1 {
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,

    sr: u8,
    pb: [u8; 4],

    uses_chr_ram: bool,

}

impl Mapper for MMC1 {
    fn cpu_write(&mut self, addr: u16, value: u8) {
        if addr < 0x8000 {
            return;
        }

        let bit = (value & 1) as u8;
        let clear = (value & 0x80) != 0;

        if clear {
            self.pb[0] = self.pb[0] | 0x0C;
            self.sr = 1 << 4;

        } else {

            // SR is not full, fill it with the 0th bit
            if self.sr & 1 == 0 {
                self.sr = (self.sr >> 1) | (bit << 4);
                // Shift register only holds 5 bits, so we must clear bit 5
                self.sr &= !(1 << 5);
            } else {
                // SR is full, copy bit 0 + SR to bank register
                // Only B13 and B14 of address is relevant
                let pb_idx = ((addr >> 13) & 0x3) as usize;
                let val = (self.sr >> 1) | (bit << 4);
                self.pb[pb_idx] = val;
                // Reset SR
                self.sr = 1 << 4;
            }
        }

    }

    fn nametable_arrangement(&self) -> NametableArrangement {
        let ret = match self.pb[0] & 0x3 {
            0 => NametableArrangement::OneScreen(0),
            1 => NametableArrangement::OneScreen(1),
            2 => NametableArrangement::Horizontal,
            3 => NametableArrangement::Vertical,
            _ => panic!("Impossible")
        };
        ret
    }

    fn cpu_read(&mut self, addr: u16) -> u8 { 
        if addr < 0x8000 {
            return 0;
        }
        let addr = addr as u32 & 0x7FFF;
        let bank_mode = (self.pb[0] >> 2) & 0x3;
        let bank = self.pb[3] & 0xF;
        
        let addr = match (bank_mode, addr) {
            // 32kB banking mode
            (0|1, _) => addr | (((bank as u32) >> 1) << 15),
            // Fix 8000-BFFF, rest is 16kb banks
            (2, 0x0000..0x4000) => addr,
            (2, 0x4000..=0x7FFF) => addr | ((bank as u32) << 14),
            // Fix C000-FFFF to last 32kb bank, rest is 16kb banks
            (3, 0x0000..0x4000) =>  addr | ((bank as u32) << 14),
            (3, 0x4000..=0x7FFF) => (self.prg_rom.len() as u32 - 32768) + addr,
            (_, _) => panic!("Bad pattern {}, {}", bank_mode, addr)
        };

        self.prg_rom[addr as usize]
    }

    fn ppu_read(&mut self, addr: u16) -> u8 {
        if self.uses_chr_ram {
            // No banking if using CHR RAM
            return self.chr_rom[addr as usize & 0x1FFF];
        }

        // Bank mode  0: 8kB, 1: 4kB
        // Ignore low bit in 8kB mode
        let bank_mode_8kb = (self.pb[0] & 0x10) >> 4;
        
        // Upper or lower address space maps to pb reg 1 or 2
        // Reg 2 is ignored in 8kb Mode
        let idx = if bank_mode_8kb != 0 {
            ((addr & 0x1000) >> 12) + 1
        }  else {
            1
        };


        // Bank, mask out low bit depending on bank mode
        let bank = (self.pb[idx as usize] & 0x1F) >> (bank_mode_8kb ^ 1);

        let addr = (((bank as u32) << 12) | (addr as u32 & 0x0FFF)) as usize;
        let ret = self.chr_rom[addr];
        ret
    }

    fn ppu_write(&mut self, addr: u16, data: u8) {
        if self.uses_chr_ram {
            self.chr_rom[addr as usize & 0x1FFF] = data;
        }

    }
}

impl MMC1 {
    pub fn new(data: NESData) -> Self {
        // Deduce if using CHR_RAM or CHR_ROM
        let chr_rom = if data.header.chr_ram_size > 0 {
            vec![0u8; data.header.chr_ram_size as usize]
        } else {
            data.chr_rom 
        };

        let mut pb = [0u8; 4];
        pb[0] = pb[0] | 0x0C;

        Self {
            chr_rom,
            prg_rom: data.prg_rom,
            sr: (1 << 4),
            pb: pb,
            uses_chr_ram: data.header.chr_ram_size > 0,
        }
    }
}

