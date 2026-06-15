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

impl CPU {

    pub fn new() -> CPU {
        return Self{
            acc: 0,
            x: 0,
            y: 0,
            pc: 0xFFFC,
            p: 0,
            s: 0xFD,
            flags: CPUFlags::IRQ,
        };
    }


    pub fn flag_as_u8(&self, status: CPUFlags) -> u8 { self.flags.contains(status) as u8 }
    pub fn get_flag(&self, status: CPUFlags) -> bool { self.flags.contains(status) }

    pub fn set_flag(&mut self, status: CPUFlags, cond: bool) {
        self.flags.set(status, cond);
    }


    // Based on result, set ZERO and NEGATIVE flags.
    // Such a common operation that it is factored out
    pub fn set_zn(&mut self, value: u8) {
        self.set_flag(CPUFlags::ZERO, value == 0);
        self.set_flag(CPUFlags::NEGATIVE, (value & 0x80) != 0);
    }


}
