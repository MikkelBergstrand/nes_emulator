mod vertex;
mod texture;
mod app;
mod addressing;
mod opcodes;
mod memory;
mod cpu;
mod instruction;
mod nes;

use app::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()?;
    Ok(())
}
