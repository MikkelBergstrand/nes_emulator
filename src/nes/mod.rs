mod nes;
mod memory;
mod addressing;

use image::Rgb;

use crate::cpu::CPU; 
use crate::nes_parser::NESData;
use crate::ram::RAM;
use crate::rom::ROM;
use crate::ppu::PPU;
use crate::opcodes::InstructionData;
use crate::nes_parser;

pub struct NES {
    cpu: CPU,
    ppu: PPU,
    ram: RAM,
    rom: ROM,
    instruction_data: [InstructionData; 256],
    cycles: usize,
}

impl NES {
    pub fn new(nes_data: NESData, color_data: &[Rgb<u8>]) -> Self {
        let mut nes = Self {
            cpu: CPU::new(),
            ppu: PPU::new(&nes_data.chr_rom, nes_data.header.nametable_arrangement, color_data),
            ram: RAM::new(),
            rom: ROM::new(&nes_data.prg_rom),
            instruction_data: InstructionData::make_instruction_table(),
            cycles: 0,
        };
        nes.cpu.pc = nes.read_u16(0xFFFC);
        return nes
    } 

    pub fn from_args() -> Self {
        dbg!("From args");
        let rom_file = std::env::args().nth(1).unwrap_or(String::from("smb1.nes"));

        let color_data_file = std::env::args().nth(2).unwrap_or(String::from("colors.pal"));

        let mut color_data: Vec<u8> = std::fs::read(&color_data_file).unwrap();
        color_data.truncate(192);

        dbg!(color_data.len());
        let color_data: Vec<Rgb<u8>> = color_data.chunks_exact(3).map(|c| Rgb([c[0], c[1], c[2]])).collect();

        let nes_data = nes_parser::read(&rom_file).unwrap();
        dbg!(nes_data.prg_rom.len());

       NES::new(nes_data, &color_data)
    }

    pub fn get_image_bytes(&mut self) -> &Vec<u8> { self.ppu.get_image_bytes() }

    pub fn image_ready(&self) -> bool { return self.ppu.image_ready(); }
}
