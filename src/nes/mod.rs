mod nes;
mod memory;
mod addressing;

use crate::cpu::CPU; 
use crate::ram::RAM;
use crate::ppu::PPU;
use crate::opcodes::InstructionData;

pub struct NES {
    cpu: CPU,
    ppu: PPU,
    ram: RAM,
    instruction_data: [InstructionData; 256],
    cycles: usize,
}

impl NES {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            ram: RAM::new(),
            instruction_data: InstructionData::make_instruction_table(),
            cycles: 0,
        }
    } 
}
