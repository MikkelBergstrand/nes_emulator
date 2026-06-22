use crate::nes::nes_parser::{NESData, NametableArrangement};

mod mapper_0;
mod mapper_1;

pub trait Mapper {
    fn cpu_write(&mut self, _addr: u16, _value: u8) {}
    fn cpu_read(&mut self, addr: u16) -> u8;

    fn ppu_write(&mut self, _addr: u16, _value: u8) {}
    fn ppu_read(&mut self, addr: u16) -> u8;

    fn nametable_arrangement(&self) -> NametableArrangement;
}

pub fn get_mapper(nes_data: NESData) -> Box<dyn Mapper> {
    match nes_data.header.mapper {
        0 => Box::new(mapper_0::NROM::new(nes_data)),
        1 => Box::new(mapper_1::MMC1::new(nes_data)),
        _ => panic!("Unsupported mapper")
    }
}

