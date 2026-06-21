mod vertex;
mod texture;
mod addressing;
mod opcodes;
mod ram;
mod rom;
mod cpu;
mod ppu;
mod instruction;
mod nes;
mod nes_parser;
mod app;
mod inputs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()?;
    Ok(())
}
