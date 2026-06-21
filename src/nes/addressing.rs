use std::fmt::format;

use super::NES;
use crate::addressing::{AddressingMode, address_crosses_page};

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
                let arg1 = self.read(address.wrapping_add(self.cpu.x as u16)) as u16;
                let arg2 = self.read(address.wrapping_add(self.cpu.x as u16).wrapping_add(1) as u16) as u16;
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
