use std::fmt::format;

use super::NES;
use crate::addressing::{AddressingMode, address_crosses_page};

impl NES {
    pub fn resolve_address(&mut self, mode: AddressingMode, address: u16) -> Option<(u16, bool)> {
        fn check_crosses_page(address: u16) -> Option<(u16, bool)>  {
            return Some((address, address_crosses_page(address)));
        }

        let address_u8 = address as u8;
        match mode {
            AddressingMode::ZeroPage => { return Some((address, false)) },
            AddressingMode::ZeroPageX => { return Some(((address + (self.cpu.x as u16)) % 256, false)) },
            AddressingMode::ZeroPageY => { return Some(((address + (self.cpu.y as u16)) % 256, false)) },
            AddressingMode::Absolute => {  return Some((address, false)) },
            AddressingMode::AbsoluteX => { return check_crosses_page(address.wrapping_add(self.cpu.x as u16)) },
            AddressingMode::AbsoluteY => { return check_crosses_page(address.wrapping_add(self.cpu.y as u16)) },
            AddressingMode::IndirectX => { 
                let arg1 = self.read(address_u8.wrapping_add(self.cpu.x) as u16) as u16;
                let arg2 = self.read(address_u8.wrapping_add(self.cpu.x).wrapping_add(1) as u16) as u16;
                Some((arg1 + (arg2 << 8), false))
            },
            AddressingMode::IndirectY => {
                let arg1 = self.read(address) as u16;
                let arg2 = self.read(address.wrapping_add(1) % 256) as u16;
                check_crosses_page(arg1 + (arg2 << 8) + (self.cpu.y as u16))
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
