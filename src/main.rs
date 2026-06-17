mod vertex;
mod texture;
mod app;
mod addressing;
mod opcodes;
mod ram;
mod cpu;
mod ppu;
mod instruction;
mod nes;
mod nes_parser;

use app::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // app::run()?;
    match nes_parser::read("smb1.nes") {
        Ok(_) => {}
        Err(e) => { println!("Error parsing ROM"); }

    }
    Ok(())
}
