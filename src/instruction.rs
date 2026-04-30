
#[derive(Copy, Clone)]
pub enum Instruction {
    ERR,
    ADC,
    AND,
    NOP,
}

impl Instruction {
    pub fn name(&self) -> &'static str {
        match self {
            Instruction::ERR => "???",
            Instruction::ADC => "adc",
            Instruction::AND => "and",
            Instruction::NOP => "nop", }
    }
}
