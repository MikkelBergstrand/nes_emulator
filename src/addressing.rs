use crate::memory::Memory;
use crate::cpu::CPU;

// How to interpret the bytes following an instruction.
// Different address modes might address the CPU, memory, or treat the bytes
// as a raw value.
#[derive(Copy, Clone, PartialEq)]
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

impl AddressingMode {
    pub fn resolve_ref_from_addressmode<'a>(mode: AddressingMode, arg: u16, cpu: &'a mut CPU, memory: &'a mut Memory) -> &'a mut u8 {
        match mode {
            AddressingMode::Accumulator => &mut cpu.acc,
            _ => {
                let (address, _) = AddressingMode::to_address(mode, arg, cpu, memory)
                    .expect("Invalid address mode");
                &mut memory[address]
            }
        }
    }

    pub fn resolve_value_from_addressmode(mode: AddressingMode, arg: Option<u16>, cpu: &CPU, memory: &Memory) -> (u8, bool) {
        match mode {
            AddressingMode::Accumulator => (cpu.acc, false),
            AddressingMode::Immediate => { (arg.expect("Immediate address mode requires an argument") as u8, false)},
            _ => {
                let arg = arg.expect("Address mode requires an argument");
                let (addr, extra_cycle) = AddressingMode::to_address(mode, arg, cpu, memory)
                    .expect("Invalid opcode. Cannot request address alongside this addressing mode.");
                (memory[addr], extra_cycle)
            }
        }
    }

    pub fn to_address(mode: AddressingMode, address: u16, cpu: &CPU, memory: &Memory) -> Option<(u16, bool)> {
        fn check_crosses_page(address: u16) -> Option<(u16, bool)>  {
            return Some((address, address_crosses_page(address)));
        }

        let address_u8 = address as u8;
        match mode {
            AddressingMode::ZeroPage => { return Some((address, false)) },
            AddressingMode::ZeroPageX => { return Some(((address + (cpu.x as u16)) % 256, false)) },
            AddressingMode::ZeroPageY => { return Some(((address + (cpu.y as u16)) % 256, false)) },
            AddressingMode::AbsoluteX => { return check_crosses_page(address.wrapping_add(cpu.x as u16)) },
            AddressingMode::AbsoluteY => { return check_crosses_page(address.wrapping_add(cpu.y as u16)) },
            AddressingMode::IndirectX => { 
                let arg1 = memory[address_u8.wrapping_add(cpu.x) as u16] as u16;
                let arg2 = memory[address_u8.wrapping_add(cpu.x).wrapping_add(1) as u16] as u16 * 256;
                Some((arg1 + arg2, false))
            },
            AddressingMode::IndirectY => {
                let arg1 = memory[address] as u16;
                let arg2 = memory[address.wrapping_add(1) as u16] as u16 * 256;
                check_crosses_page(arg1 + arg2 + cpu.y as u16)
            }
            _ => { return None; }
        }
    }
}


// Determine if the address above the input address is in another page.
// For instance, the address 0x03FF and 0x0400 are in different pages.
// The check boils down to checking if the two LSB are FF.
pub fn address_crosses_page(address: u16) -> bool {
    (address & 0xFF) == 0xFF
}

