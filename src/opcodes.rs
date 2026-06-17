use crate::instruction::Instruction;
use crate::addressing::AddressingMode;



#[derive(Copy, Clone, Debug)]
pub struct InstructionData {
    pub address_mode: AddressingMode,
    pub instruction: Instruction,
    pub bytes: u8,
    pub cycles: u8,
}


macro_rules! op {
    ($data:ident, $opcode:expr, $mode:ident, $instr:ident, $bytes:expr, $cycles:expr) => {
        $data[$opcode] = InstructionData {
            address_mode: AddressingMode::$mode,
            instruction: Instruction::$instr,
            bytes: $bytes,
            cycles: $cycles,
        };
    };
}

impl InstructionData {
    const fn invalid() -> InstructionData {
        Self {
            bytes: 0,
            cycles: 0,
            instruction: Instruction::ERR,
            address_mode: AddressingMode::Implied,
        }
    }

    pub const fn make_instruction_table() -> [InstructionData; 256] {
        let mut data: [InstructionData; 256] = [InstructionData::invalid(); 256];

        // ADC
        op!(data, 0x69, Immediate, ADC, 2, 2);
        op!(data, 0x65, ZeroPage,  ADC, 2, 3);
        op!(data, 0x75, ZeroPageX, ADC, 2, 4);
        op!(data, 0x6D, Absolute,  ADC, 3, 4);
        op!(data, 0x7D, AbsoluteX, ADC, 3, 4);
        op!(data, 0x79, AbsoluteY, ADC, 3, 4);
        op!(data, 0x61, IndirectX, ADC, 2, 6);
        op!(data, 0x71, IndirectY, ADC, 2, 5);

        // SBC 
        op!(data, 0xE9, Immediate, SBC, 2, 2);
        op!(data, 0xE5, ZeroPage,  SBC, 2, 3);
        op!(data, 0xF5, ZeroPageX, SBC, 2, 4);
        op!(data, 0xED, Absolute,  SBC, 3, 4);
        op!(data, 0xFD, AbsoluteX, SBC, 3, 4);
        op!(data, 0xF9, AbsoluteY, SBC, 3, 4);
        op!(data, 0xE1, IndirectX, SBC, 2, 6);
        op!(data, 0xF1, IndirectY, SBC, 2, 5);

        // AND
        op!(data, 0x29, Immediate, AND, 2, 2);
        op!(data, 0x25, ZeroPage,  AND, 2, 3);
        op!(data, 0x35, ZeroPageX, AND, 2, 4);
        op!(data, 0x2D, Absolute,  AND, 3, 4);
        op!(data, 0x3D, AbsoluteX, AND, 3, 4);
        op!(data, 0x39, AbsoluteY, AND, 3, 4);
        op!(data, 0x21, IndirectX, AND, 2, 6);
        op!(data, 0x31, IndirectY, AND, 2, 5);

        // ORA
        op!(data, 0x09, Immediate, ORA, 2, 2);
        op!(data, 0x05, ZeroPage,  ORA, 2, 3);
        op!(data, 0x15, ZeroPageX, ORA, 2, 4);
        op!(data, 0x0D, Absolute,  ORA, 3, 4);
        op!(data, 0x1D, AbsoluteX, ORA, 3, 4);
        op!(data, 0x19, AbsoluteY, ORA, 3, 4);
        op!(data, 0x01, IndirectX, ORA, 2, 6);
        op!(data, 0x11, IndirectY, ORA, 2, 5);

        // ASL
        op!(data, 0x0A, Accumulator, ASL, 1, 2);
        op!(data, 0x06, ZeroPage,    ASL, 2, 5);
        op!(data, 0x16, ZeroPageX,   ASL, 2, 6);
        op!(data, 0x0E, Absolute,    ASL, 3, 6);
        op!(data, 0x1E, AbsoluteX,   ASL, 3, 7);

        // BCC
        op!(data, 0x90, Relative, BCC, 2, 2);

        //BIT
        op!(data, 0x24, ZeroPage, BIT, 2, 3);
        op!(data, 0x2C, Absolute, BIT, 3, 4);

        //BMI
        op!(data, 0x30, Relative, BIT, 2, 2);

        //BNE
        op!(data, 0xD0, Relative, BNE, 2, 2);

        //BPL
        op!(data, 0x10, Relative, BPL, 2, 2);

        //BRK
        op!(data, 0x00, Implied, BRK, 2, 7);

        //BVC
        op!(data, 0x50, Relative, BVC, 2, 2);

        //BVS
        op!(data, 0x70, Relative, BVS, 2, 2);

        //CLC
        op!(data, 0x18, Implied, CLC, 1, 2);

        //CLD
        op!(data, 0xD8, Implied, CLD, 1, 2);

        //CLI
        op!(data, 0x58, Implied, CLI, 1, 2);

        //CLV
        op!(data, 0xB8, Implied, CLV, 1, 2);

        //CMP
        op!(data, 0xC9, Immediate, CMP, 2, 2);
        op!(data, 0xC5, ZeroPage,  CMP, 2, 3);
        op!(data, 0xD5, ZeroPageX, CMP, 2, 4);
        op!(data, 0xCD, Absolute,  CMP, 3, 4);
        op!(data, 0xDD, AbsoluteX, CMP, 3, 4);
        op!(data, 0xD9, AbsoluteY, CMP, 3, 4);
        op!(data, 0xC1, IndirectX, CMP, 2, 6);
        op!(data, 0xD1, IndirectY, CMP, 2, 5);

        //CPX
        op!(data, 0xE0, Immediate, CPX, 2, 2);
        op!(data, 0xE4, ZeroPage,  CPX, 2, 3);
        op!(data, 0xEC, Absolute,  CPX, 3, 4);

        //CPY
        op!(data, 0xC0, Immediate, CPY, 2, 2);
        op!(data, 0xC4, ZeroPage,  CPY, 2, 3);
        op!(data, 0xCC, Absolute,  CPY, 3, 4);

        //DEC
        op!(data, 0xC6, ZeroPage,   DEC, 2, 5);
        op!(data, 0xD6, ZeroPageX,  DEC, 2, 6);
        op!(data, 0xCE, Absolute,   DEC, 3, 6);
        op!(data, 0xDE, AbsoluteX,  DEC, 3, 7);

        //DEX
        op!(data, 0xCA, Implied, DEX, 1, 2);

        //DEY
        op!(data, 0x88, Implied, DEY, 1, 2);

        //EOR
        op!(data, 0x49, Immediate, EOR, 2, 2);
        op!(data, 0x45, ZeroPage,  EOR, 2, 3);
        op!(data, 0x55, ZeroPageX, EOR, 2, 4);
        op!(data, 0x4D, Absolute,  EOR, 3, 4);
        op!(data, 0x5D, AbsoluteX, EOR, 3, 4);
        op!(data, 0x59, AbsoluteY, EOR, 3, 4);
        op!(data, 0x41, IndirectX, EOR, 2, 6);
        op!(data, 0x51, IndirectY, EOR, 2, 5);
        
        //INC
        op!(data, 0xE6, ZeroPage,   INC, 2, 5);
        op!(data, 0xF6, ZeroPageX,  INC, 2, 6);
        op!(data, 0xEE, Absolute,   INC, 3, 6);
        op!(data, 0xFE, AbsoluteX,  INC, 3, 7);

        //INX
        op!(data, 0xE8, Implied, INX, 1, 2);

        //INY
        op!(data, 0xC8, Implied, INY, 1, 2);

        //JMP
        op!(data, 0x4C, Absolute, JMP, 3, 5);
        op!(data, 0x6C, Indirect, JMP, 3, 5);

        //JSR
        op!(data, 0x20, Absolute, JSR, 3, 6);

        //LDA
        op!(data, 0xA9, Immediate, LDA, 2, 2);
        op!(data, 0xA5, ZeroPage,  LDA, 2, 3);
        op!(data, 0xB5, ZeroPageX, LDA, 2, 4);
        op!(data, 0xAD, Absolute,  LDA, 3, 4);
        op!(data, 0xBD, AbsoluteX, LDA, 3, 4);
        op!(data, 0xB9, AbsoluteY, LDA, 3, 4);
        op!(data, 0xA1, IndirectX, LDA, 2, 6);
        op!(data, 0xB1, IndirectY, LDA, 2, 5);

        //LDX
        op!(data, 0xA2, Immediate,  LDX, 2, 2);
        op!(data, 0xA6, ZeroPage,   LDX, 2, 3);
        op!(data, 0xB6, ZeroPageY,  LDX, 2, 4);
        op!(data, 0xAE, Absolute,   LDX, 3, 4);
        op!(data, 0xBE, AbsoluteY,  LDX, 3, 4);

        //LDY
        op!(data, 0xA0, Immediate,  LDY, 2, 2);
        op!(data, 0xA4, ZeroPage,   LDY, 2, 3);
        op!(data, 0xB4, ZeroPageX,  LDY, 2, 4);
        op!(data, 0xAC, Absolute,   LDY, 3, 4);
        op!(data, 0xBC, AbsoluteX,  LDY, 3, 4);

        //LSR
        op!(data, 0x4A, Accumulator,LSR, 1, 2);
        op!(data, 0x46, ZeroPage,   LSR, 2, 5);
        op!(data, 0x56, ZeroPageX,  LSR, 2, 6);
        op!(data, 0x4E, Absolute,   LSR, 3, 6);
        op!(data, 0x5E, AbsoluteX,  LSR, 3, 7);

        //ROL
        op!(data, 0x2A, Accumulator,ROL, 1, 2);
        op!(data, 0x26, ZeroPage,   ROL, 2, 5);
        op!(data, 0x36, ZeroPageX,  ROL, 2, 6);
        op!(data, 0x2E, Absolute,   ROL, 3, 6);
        op!(data, 0x3E, AbsoluteX,  ROL, 3, 7);

        //ROR
        op!(data, 0x6A, Accumulator,ROR, 1, 2);
        op!(data, 0x66, ZeroPage,   ROR, 2, 5);
        op!(data, 0x76, ZeroPageX,  ROR, 2, 6);
        op!(data, 0x6E, Absolute,   ROR, 3, 6);
        op!(data, 0x7E, AbsoluteX,  ROR, 3, 7);

        //NOP
        op!(data, 0xEA, Implied,    NOP, 1, 2);

        //PHA
        op!(data, 0x48, Implied,    PHA, 1, 3);

        //PHP
        op!(data, 0x08, Implied,    PHP, 1, 3);

        //PLA
        op!(data, 0x68, Implied,    PLA, 1, 4);

        //PLP
        op!(data, 0x28, Implied,    PLP, 1, 4);

        //RTI
        op!(data, 0x40, Implied,    RTI, 1, 6);

        //RTS
        op!(data, 0x60, Implied,    RTS, 1, 6);

        //SEC
        op!(data, 0x38, Implied,    SEC, 1, 2);

        //SED
        op!(data, 0xF8, Implied,    SED, 1, 2);

        //SEI
        op!(data, 0x78, Implied,    SEI, 1, 2);
        
        //STA
        op!(data, 0x85, ZeroPage,  STA, 2, 3);
        op!(data, 0x95, ZeroPageX, STA, 2, 4);
        op!(data, 0x8D, Absolute,  STA, 3, 4);
        op!(data, 0x9D, AbsoluteX, STA, 3, 5);
        op!(data, 0x99, AbsoluteY, STA, 3, 5);
        op!(data, 0x81, IndirectX, STA, 2, 6);
        op!(data, 0x91, IndirectY, STA, 2, 6);

        //STX
        op!(data, 0x86, ZeroPage,   STX, 2, 3);
        op!(data, 0x96, ZeroPageY,  STX, 2, 4);
        op!(data, 0x8E, Absolute,   STX, 3, 4);

        //STY
        op!(data, 0x84, ZeroPage,   STY, 2, 3);
        op!(data, 0x94, ZeroPageX,  STY, 2, 4);
        op!(data, 0x8C, Absolute,   STY, 3, 4);

        //TAX
        op!(data, 0xAA, Implied,   TAX, 1, 2);

        //TAY
        op!(data, 0xA8, Implied,   TAY, 1, 2);

        //TSX
        op!(data, 0xBA, Implied,   TSX, 1, 2);

        //TXA
        op!(data, 0x8A, Implied,   TXA, 1, 2);

        //TXS
        op!(data, 0x9A, Implied,   TXS, 1, 2);

        //TYA
        op!(data, 0x98, Implied,   TYA, 1, 2);

        return data;
    }
}







