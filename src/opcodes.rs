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

        // AND
        op!(data, 0x29, Immediate, AND, 2, 2);
        op!(data, 0x25, ZeroPage,  AND, 2, 3);
        op!(data, 0x35, ZeroPageX, AND, 2, 4);
        op!(data, 0x2D, Absolute,  AND, 3, 4);
        op!(data, 0x3D, AbsoluteX, AND, 3, 4);
        op!(data, 0x39, AbsoluteY, AND, 3, 4);
        op!(data, 0x21, IndirectX, AND, 2, 6);
        op!(data, 0x31, IndirectY, AND, 2, 5);

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

        return data;
    }
}







