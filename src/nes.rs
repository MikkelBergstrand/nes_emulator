use core::panic;

use bytemuck::offset_of;
use wgpu::AddressMode;

use crate::cpu::{CPU, CPUFlags}; 
use crate::instruction::{self, Instruction};
use crate::memory::Memory;
use crate::opcodes::{self, AddressingMode, InstructionData};

pub struct NES {
    cpu: CPU,
    memory: Memory,
    instruction_data: [InstructionData; 256],
}

impl NES {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            memory: Memory::new(),
            instruction_data: opcodes::InstructionData::make_instruction_table(),
        }
    } 


    fn branch_if(&mut self, condition: bool, addr: Option<u16>) {
        if condition {
            let offset = addr.unwrap() as i8;
            self.cpu.pc = self.cpu.pc.wrapping_add(2).wrapping_add(offset as i16 as u16);
        }
    }

    fn compare_reg(&mut self, mode: AddressingMode, addr: Option<u16>, register: u8) {
        let value = AddressingMode::resolve_value_from_addressmode(mode, addr, &mut self.cpu, &mut self.memory);
        self.cpu.set_flag(CPUFlags::CARRY, register >= value);
        self.cpu.set_flag(CPUFlags::ZERO, register == value);
        self.cpu.set_flag(CPUFlags::NEGATIVE, (value & 0x80) != 0);
    }

    fn shift_rmw(&mut self, mode: AddressingMode, addr: Option<u16>, op: impl Fn(u8, bool) -> (u8, bool)) {
        let value = AddressingMode::resolve_value_from_addressmode(mode, addr, &self.cpu, &self.memory);
        let (result, carry) = op(value, self.cpu.get_flag(CPUFlags::CARRY));
        self.cpu.set_flag(CPUFlags::CARRY, carry);
        self.cpu.set_flag(CPUFlags::ZERO, result == 0);
        self.cpu.set_flag(CPUFlags::NEGATIVE, (result & 0x80) != 0);
        *AddressingMode::resolve_ref_from_addressmode(mode, addr.unwrap(), &mut self.cpu, &mut self.memory) = result;
    }

    pub fn tick(&mut self) {
        let opcode = self.memory[self.cpu.pc];
        let instruction_data = self.instruction_data[opcode as usize];
        let addr_mode = instruction_data.address_mode;
        
        let addr: Option<u16> = match instruction_data.bytes {
            1 => None,
            2 => Some(self.memory[self.cpu.pc.wrapping_add(1)] as u16),
            3 => Some(((self.memory[self.cpu.pc.wrapping_add(1)] as u16) >> 8) + (self.memory[self.cpu.pc.wrapping_add(2)] as u16)),
            _ => panic!("Invalid number of bytes for opcode.")
        };

        // execute instruction...
        match instruction_data.instruction {
            Instruction::ADC => {
                let memory = AddressingMode::resolve_value_from_addressmode(instruction_data.address_mode, addr, &mut self.cpu, &mut self.memory);

                // A = A + memory + C
                // Detect overflow to set the carry bit. Bit hacky, but merge two overflowing_add
                // operations
                let (result, wrapped_a) = self.cpu.acc.overflowing_add(memory);
                let (result, wrapped_b) = result.overflowing_add(self.cpu.flag_as_u8(CPUFlags::CARRY));
                let wrapped = wrapped_a | wrapped_b;

                self.cpu.set_flag(CPUFlags::CARRY, wrapped);
                self.cpu.set_flag(CPUFlags::ZERO, result == 0);
                self.cpu.set_flag(CPUFlags::OVERFLOW, ((result ^ self.cpu.acc) & (result ^ memory) & 0x80) != 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (result & 0x80) != 0);

                self.cpu.acc = result;
                
            }
            Instruction::AND => {
                let memory = AddressingMode::resolve_value_from_addressmode(instruction_data.address_mode, addr, &mut self.cpu, &mut self.memory);

                self.cpu.acc = self.cpu.acc & memory;
                self.cpu.set_flag(CPUFlags::ZERO, self.cpu.acc == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (self.cpu.acc & 0x80) != 0);
            }
            Instruction::ASL => self.shift_rmw(addr_mode, addr, |v, _| (v << 1, (v & 0x80) != 0)),
            Instruction::LSR => self.shift_rmw(addr_mode, addr, |v, _| (v >> 1, (v & 0x01) != 0)),
            Instruction::ROL => self.shift_rmw(addr_mode, addr, |v, c| ((v << 1) | c as u8,        (v & 0x80) != 0)),
            Instruction::ROR => self.shift_rmw(addr_mode, addr, |v, c| ((v >> 1) | (c as u8) << 7, (v & 0x01) != 0)),
            Instruction::BCC => { self.branch_if(self.cpu.get_flag(CPUFlags::CARRY), addr); },
            Instruction::BEQ => { self.branch_if(self.cpu.get_flag(CPUFlags::ZERO), addr); },
            Instruction::BIT => {
                let value = AddressingMode::resolve_value_from_addressmode(addr_mode, addr, &mut self.cpu, &mut self.memory);
                self.cpu.set_flag(CPUFlags::ZERO, value == 0);
                self.cpu.set_flag(CPUFlags::OVERFLOW, (value & 0x40) != 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (value & 0x80) != 0);
            }
            Instruction::BMI => { self.branch_if(self.cpu.get_flag(CPUFlags::NEGATIVE), addr); },
            Instruction::BNE => { self.branch_if(!self.cpu.get_flag(CPUFlags::ZERO), addr); },
            Instruction::BPL => { self.branch_if(!self.cpu.get_flag(CPUFlags::NEGATIVE), addr); },
            Instruction::BVC => { self.branch_if(!self.cpu.get_flag(CPUFlags::OVERFLOW), addr); },
            Instruction::BVS => { self.branch_if(self.cpu.get_flag(CPUFlags::OVERFLOW), addr); },
            Instruction::CLC => { self.cpu.set_flag(CPUFlags::CARRY, false); },
            Instruction::CLD => { self.cpu.set_flag(CPUFlags::DECIMAL, false); },
            Instruction::CLI => { self.cpu.set_flag(CPUFlags::IRQ, false); },
            Instruction::CLV => { self.cpu.set_flag(CPUFlags::OVERFLOW, false); },
            Instruction::SEC => { self.cpu.set_flag(CPUFlags::CARRY, true); },
            Instruction::SED => { self.cpu.set_flag(CPUFlags::DECIMAL, true); },
            Instruction::SEI => { self.cpu.set_flag(CPUFlags::IRQ, true); },
            Instruction::CMP => { self.compare_reg(addr_mode, addr, self.cpu.acc); }
            Instruction::CPX => { self.compare_reg(addr_mode, addr, self.cpu.x); }
            Instruction::CPY => { self.compare_reg(addr_mode, addr, self.cpu.y); }
            Instruction::DEC => { 
                let addr = AddressingMode::to_address(addr_mode, addr.unwrap(), &self.cpu, &self.memory).unwrap();
                let result = self.memory[addr].wrapping_sub(1);
                self.memory[addr] = result;

                self.cpu.set_flag(CPUFlags::ZERO, result == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (result & 0x80) == 0);
            }
            Instruction::DEX => {
                self.cpu.x = self.cpu.x.wrapping_sub(1);
                self.cpu.set_flag(CPUFlags::ZERO, self.cpu.x == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (self.cpu.x & 0x80) == 0);
            }
            Instruction::DEY => {
                self.cpu.y = self.cpu.y.wrapping_sub(1);
                self.cpu.set_flag(CPUFlags::ZERO, self.cpu.y == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (self.cpu.y & 0x80) == 0);
            }
            Instruction::INC => { 
                let addr = AddressingMode::to_address(addr_mode, addr.unwrap(), &self.cpu, &self.memory).unwrap();
                let result = self.memory[addr].wrapping_add(1);
                self.memory[addr] = result;

                self.cpu.set_flag(CPUFlags::ZERO, result == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (result & 0x80) == 0);
            }
            Instruction::INX => {
                self.cpu.x = self.cpu.x.wrapping_add(1);
                self.cpu.set_flag(CPUFlags::ZERO, self.cpu.x == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (self.cpu.x & 0x80) == 0);
            }
            Instruction::INY => {
                self.cpu.y = self.cpu.y.wrapping_add(1);
                self.cpu.set_flag(CPUFlags::ZERO, self.cpu.y == 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE, (self.cpu.y & 0x80) == 0);
            }
            Instruction::NOP => {
                 
            }
            _ => panic!("unimplemented instruction {}", instruction_data.instruction)
        }

        // advance program counter
        self.cpu.pc = self.cpu.pc.wrapping_add(instruction_data.bytes as u16);

    }
}
