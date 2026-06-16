mod flags;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct CPUFlags: u8 {
        const CARRY    = 1 << 0;
        const ZERO     = 1 << 1;
        const IRQ      = 1 << 2;
        const DECIMAL  = 1 << 3;
        const BREAK    = 1 << 4;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7;
    }
}

pub struct CPU {
    // Registers
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: u8,
    pub flags: CPUFlags,
}


