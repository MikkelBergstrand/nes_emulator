use super::NES;

// How to interpret the bytes following an instruction.
// Different address modes might address the CPU, memory, or treat the bytes
// as a raw value.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AddressingMode {
    Immediate,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect, // Used by JMP
    IndirectX,
    IndirectY,
    Accumulator,
    Relative
}

// True when adding an index moved the effective address into a different
// 256-byte page (i.e. the high byte changed). Used to charge the extra cycle
// for AbsoluteX/AbsoluteY/IndirectY reads.
pub fn address_crosses_page(base: u16, effective: u16) -> bool {
    (base & 0xFF00) != (effective & 0xFF00)
}


impl NES {
    pub fn resolve_address(&mut self, mode: AddressingMode, address: u16) -> Option<(u16, bool)> {
        match mode {
            AddressingMode::ZeroPage => { return Some((address, false)) },
            AddressingMode::ZeroPageX => { return Some(((address + (self.cpu.x as u16)) % 256, false)) },
            AddressingMode::ZeroPageY => { return Some(((address + (self.cpu.y as u16)) % 256, false)) },
            AddressingMode::Absolute => {  return Some((address, false)) },
            AddressingMode::AbsoluteX => { 
                let eff = address.wrapping_add(self.cpu.x as u16);
                return Some((eff, address_crosses_page(address, eff)));
            },
            AddressingMode::AbsoluteY => { 
                let eff = address.wrapping_add(self.cpu.y as u16);
                return Some((eff, address_crosses_page(address, eff)));
            }
            AddressingMode::IndirectX => { 
                let ptr = (address as u8).wrapping_add(self.cpu.x) as u16;
                let arg1 = self.read(ptr) as u16;
                let arg2 = self.read((ptr+1) & 0xFF) as u16;
                Some((arg1 | (arg2 << 8), false))
            },
            AddressingMode::IndirectY => {
                let lo = self.read(address) as u16;
                let hi = self.read(address.wrapping_add(1) % 256) as u16;
                let ptr = (hi << 8) | lo;
                let eff = ptr.wrapping_add(self.cpu.y as u16);
                return Some((eff, address_crosses_page(ptr, eff)));
            }
            _ => { return None; }
        }
    }

    pub fn resolve_value_from_addressmode(&mut self, mode: AddressingMode, arg: Option<u16>) -> (u8, bool) {
        match mode {
            AddressingMode::Accumulator => (self.cpu.acc, false),
            AddressingMode::Immediate => (arg.unwrap() as u8, false),
            _ => {
                let arg = arg.expect("Address mode requires an argument");
                let (addr, extra_cycle) = self.resolve_address(mode, arg)
                    .unwrap_or_else(|| panic!("Invalid opcode, addr: {:?}. Cannot request address alongside addressing mode {:?}", arg, mode));

                (self.read(addr), extra_cycle)
            }
        }
    }
}
