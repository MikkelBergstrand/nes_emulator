mod vertex;
mod texture;
mod app;
mod addressing;
mod opcodes;
mod ram;
mod rom;
mod cpu;
mod ppu;
mod instruction;
mod nes;
mod nes_parser;

use std::{thread::sleep, time::Duration};

use nes::NES;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_file = std::env::args().nth(1).unwrap_or(String::from("smb1.nes"));

    let nes_data = nes_parser::read(&rom_file)?;
    dbg!(nes_data.prg_rom.len());
    let mut nes = NES::new(&nes_data.prg_rom);
    loop {
        nes.tick();
        sleep(Duration::from_secs(1));
    }
    Ok(())
}
