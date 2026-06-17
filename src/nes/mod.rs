mod nes;
mod memory;
mod addressing;

use crate::cpu::CPU; 
use crate::ram::RAM;
use crate::rom::ROM;
use crate::ppu::PPU;
use crate::opcodes::InstructionData;

pub struct NES {
    cpu: CPU,
    ppu: PPU,
    ram: RAM,
    rom: ROM,
    instruction_data: [InstructionData; 256],
    cycles: usize,
}

impl NES {
    pub fn new(rom_data: &[u8]) -> Self {
        Self {
            cpu: CPU::new(),
            ppu: PPU::new(),
            ram: RAM::new(),
            rom: ROM::new(rom_data),
            instruction_data: InstructionData::make_instruction_table(),
            cycles: 0,
        }
    } 
}
