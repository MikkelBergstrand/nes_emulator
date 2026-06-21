use nes_emulator::graphics::app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()?;
    Ok(())
}
