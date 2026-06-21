use nes_emulator::app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()?;
    Ok(())
}
