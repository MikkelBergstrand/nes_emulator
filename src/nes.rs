use core::panic;

use crate::cpu::{CPU, CPUFlags}; 
use crate::instruction::Instruction;
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


    pub fn tick(&mut self) {
        let opcode = self.memory[self.cpu.pc];
        let instruction_data = self.instruction_data[opcode as usize];
        
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

            }
            Instruction::NOP => {

            }
            _ => panic!("unimplemented instruction {}", instruction_data.instruction.name())
        }

        // advance program counter
        self.cpu.pc = self.cpu.pc.wrapping_add(instruction_data.bytes as u16);

    }
}
