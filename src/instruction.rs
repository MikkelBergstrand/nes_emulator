
use std::fmt;

#[derive(Copy, Clone)]
pub enum Instruction {
    ERR,
    ADC,
    AND,
    ASL, // Arithmetic Shift Left
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
    CLC, CLD, CLI, CLV, 
    SEC, SEI, SED,
    BIT,
    CMP, CPX, CPY,
    DEC, DEX, DEY,
    INC, INX, INY,
    LSR,
    NOP,
    ROL, ROR,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Instruction::ERR => "???",
            Instruction::ADC => "adc",
            Instruction::AND => "and",
            Instruction::ASL => "asl",
            Instruction::BCC => "bcc",
            Instruction::BCS => "bcs",
            Instruction::BEQ => "beq",
            Instruction::BMI => "bmi",
            Instruction::BNE => "bne",
            Instruction::BPL => "bpl",
            Instruction::BVC => "bvc",
            Instruction::BVS => "bvs",
            Instruction::CLC => "clc",
            Instruction::CLD => "cld",
            Instruction::CLI => "cli",
            Instruction::CLV => "clv",
            Instruction::BIT => "bit",
            Instruction::CMP => "cmp",
            Instruction::CPX => "cpx",
            Instruction::CPY => "cpy",
            Instruction::DEC => "dec",
            Instruction::DEX => "dex",
            Instruction::DEY => "dey",
            Instruction::INC => "inc",
            Instruction::INX => "inx",
            Instruction::INY => "iny",
            Instruction::LSR => "lsr",
            Instruction::NOP => "nop",
            Instruction::ROL => "rol",
            Instruction::ROR => "ror",
            Instruction::SEC => "sec",
            Instruction::SED => "sed",
            Instruction::SEI => "sei",
        };
        f.write_str(s)
    }
}
