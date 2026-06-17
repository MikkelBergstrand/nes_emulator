use super::{CPU, CPUFlags};

impl CPU {
    pub fn new() -> CPU {
        return Self{
            acc: 0,
            x: 0,
            y: 0,
            pc: 0,
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
