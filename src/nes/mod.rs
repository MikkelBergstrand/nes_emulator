mod nes;
mod cpu;
mod ppu;
mod ram;
mod opcodes;
mod memory;
mod nes_parser;
mod addressing;
mod input_controller;
mod instruction;
mod mappers;

use image::Rgb;

use cpu::CPU; 
use crate::nes::input_controller::InputController;
use nes_parser::NESData;
use ram::RAM;
use ppu::PPU;
use opcodes::InstructionData;
use mappers::Mapper;

pub struct NES {
    cpu: CPU,
    ppu: PPU,
    ram: RAM,
    input_controller: InputController,
    instruction_data: [InstructionData; 256],
    cycles: usize,
    mapper: Box<dyn Mapper>,
}

impl NES {
    pub fn new(nes_data: NESData, color_data: &[Rgb<u8>]) -> Self {
        let mut nes = Self {
            cpu: CPU::new(),
            ppu: PPU::new(color_data),
            ram: RAM::new(),
            input_controller: InputController::new(),
            instruction_data: InstructionData::make_instruction_table(),
            cycles: 0,
            mapper: mappers::get_mapper(nes_data)
        };
        nes.cpu.pc = nes.read_u16(0xFFFC);
        return nes
    } 

    pub fn from_args() -> Self {
        let rom_file = std::env::args().nth(1).unwrap_or(String::from("roms/smb1.nes"));
        let color_data_file = std::env::args().nth(2).unwrap_or(String::from("colors.pal"));

        let mut color_data: Vec<u8> = std::fs::read(&color_data_file).unwrap();
        color_data.truncate(192);

        let color_data: Vec<Rgb<u8>> = color_data.chunks_exact(3).map(|c| Rgb([c[0], c[1], c[2]])).collect();

        let nes_data = nes_parser::read(&rom_file).unwrap();

       NES::new(nes_data, &color_data)
    }

    pub fn get_image_bytes(&mut self) -> &Vec<u8> { self.ppu.get_image_bytes() }

    pub fn image_ready(&self) -> bool { return self.ppu.image_ready(); }

    pub fn set_controller_state(&mut self, state: u8) {
        self.input_controller.set_controller_state(state);
    }
}
