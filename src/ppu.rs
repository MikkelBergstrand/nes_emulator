use core::panic;
use std::usize;


pub enum PPUReg {
    Ctrl = 0, Mask, Status, OamAddr, OamData,
    Scroll, Addr, Data, OamDma
}


pub struct PPU {
    data: [u8; 8],
}

impl PPU {
    pub fn new() -> Self {
        Self{
            data: [0; 8]
        }
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        self.data[addr as usize] = data;
    }

    pub fn read(&self, addr: u8) -> u8 {
        return self.data[addr as usize];
    }

    fn read_reg(&self, reg: PPUReg) -> u8 {
        return self.data[reg as usize];
    }

    fn base_nametable_addr(&self) -> u16 {
        match self.read_reg(PPUReg::Ctrl) & 0x03 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Bad base_nametable_addr")
        }
    }

    fn vram_address_mode_down(&self) -> bool { (self.read_reg(PPUReg::Status)) & (1 << 2) != 0 }
    fn sprite_pattern_table_address(&self) -> u16 { if (self.read_reg(PPUReg::Status)) & (1 << 3) != 0 { 0x1000 } else { 0 }}
    fn background_pattern_address(&self) -> u16   { if (self.read_reg(PPUReg::Status)) & (1 << 4) != 0 { 0x1000 } else { 0 }}
    fn sprite_size(&self) -> (usize, usize)   { if (self.read_reg(PPUReg::Status)) & (1 << 5) != 0 { (8, 8) } else { (8, 16) }}
    fn master_slave_select(&self) -> bool   { (self.read_reg(PPUReg::Status)) & (1 << 6) != 0 }
    fn vblank_nmi_enable(&self) -> bool   { (self.read_reg(PPUReg::Status)) & (1 << 6) != 0 }
}
