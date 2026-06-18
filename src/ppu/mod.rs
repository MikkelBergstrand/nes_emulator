pub mod pattern_table;

use core::panic;
use std::usize;


pub struct PPU {
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    oam_data: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    oam_dma: u8,
    pending_nmi: bool
}

impl PPU {
    pub fn new() -> Self {
        Self {
            data: 0,
            mask: 0,
            status: 0x80,
            oam_addr: 0,
            oam_data: 0,
            addr: 0,
            oam_dma: 0,
            ctrl: 0,
            scroll: 0,
            pending_nmi: false
        }
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        match addr {
            0 => { self.ctrl = data; }
            1 => { self.mask = data; }
            2 => { self.status = data; }
            3 => { self.oam_addr = data; }
            4 => { self.oam_data = data; }
            5 => { self.scroll = data; }
            6 => { self.addr = data; }
            7 => { self.data = data; }
            _ => panic!("Bad PPU address")
        }
    }

    pub fn read(&self, addr: u8) -> u8 {
        match addr {
            0 => { self.ctrl }
            1 => { self.mask }
            2 => { self.status }
            3 => { self.oam_addr }
            4 => { self.oam_data }
            5 => { self.scroll }
            6 => { self.addr }
            7 => { self.data }
            _ => panic!("Bad PPU address")
        }
    }

    pub fn pending_nmi(&self) -> bool { self.pending_nmi }
    pub fn clear_nmi(&mut self) { self.pending_nmi = false; }

    pub fn tick(&mut self) {
        self.status |= 0x80;

        if self.vblank_nmi_enable() {
            self.pending_nmi = true;
        }
    }


    fn base_nametable_addr(&self) -> u16 {
        match self.ctrl & 0x03 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Bad base_nametable_addr")
        }
    }

    fn vram_address_mode_down(&self) -> bool { (self.status & (1 << 2)) != 0 }
    fn sprite_pattern_table_address(&self) -> u16 { if (self.status & (1 << 3)) != 0 { 0x1000 } else { 0 }}
    fn background_pattern_address(&self) -> u16   { if (self.status & (1 << 4)) != 0 { 0x1000 } else { 0 }}
    fn sprite_size(&self) -> (usize, usize)   { if (self.status & (1 << 5)) != 0 { (8, 8) } else { (8, 16) }}
    fn master_slave_select(&self) -> bool   { (self.status & (1 << 6)) != 0 }
    fn vblank_nmi_enable(&self) -> bool   { (self.ctrl & (1 << 7)) != 0 }
}
