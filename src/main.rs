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

use std::{process::exit, thread::sleep, time::Duration};

use image::RgbImage;
use nes::NES;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_file = std::env::args().nth(1).unwrap_or(String::from("smb1.nes"));

    let nes_data = nes_parser::read(&rom_file)?;
    dbg!(nes_data.prg_rom.len());
    let mut nes = NES::new(&nes_data.prg_rom);

    let (image_bytes, w, h) = ppu::pattern_table::pattern_tables_to_bytes(&nes_data.chr_rom);
    let image = RgbImage::from_raw(w as u32, h as u32, image_bytes).unwrap();
    let _ = image.save("out.png");
    exit(0);
    loop {
        nes.tick();
    } 

    Ok(())
}
