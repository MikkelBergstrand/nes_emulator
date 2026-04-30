use crate::memory::Memory;
use crate::instruction::Instruction;
use crate::cpu::CPU;


#[derive(Copy, Clone)]
pub enum AddressingMode {
    Immediate,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

impl AddressingMode {
    pub fn resolve_value_from_addressmode(mode: AddressingMode, arg: Option<u16>, cpu: &CPU, memory: &mut Memory) -> u8 {
        match mode {
            AddressingMode::Immediate => { arg.expect("Immediate address mode requires an argument") as u8 },
            _ => {
                let arg = arg.expect("Address mode requires an argument");
                let addr = AddressingMode::to_address(mode, arg, cpu, memory)
                    .expect("Invalid opcode. Cannot request address alongside this addressing mode.");
                memory[addr]
            }
        }
    }

    fn to_address(mode: AddressingMode, address: u16, cpu: &CPU, memory: &mut Memory) -> Option<u16> {
        let address_u8 = address as u8;
        match mode {
            AddressingMode::ZeroPage => { return Some(address) },
            AddressingMode::ZeroPageX => { return Some((address + (cpu.x as u16)) % 256)},
            AddressingMode::ZeroPageY => { return Some((address + (cpu.y as u16)) % 256)},
            AddressingMode::AbsoluteX => { return Some(address.wrapping_add(cpu.x as u16)) },
            AddressingMode::AbsoluteY => { return Some(address.wrapping_add(cpu.y as u16)) },
            AddressingMode::IndirectX => { 
                let arg1 = memory[address_u8.wrapping_add(cpu.x) as u16] as u16;
                let arg2 = memory[address_u8.wrapping_add(cpu.x).wrapping_add(1) as u16] as u16 * 256;
                Some(arg1 + arg2)
            },
            AddressingMode::IndirectY => {
                let arg1 = memory[address] as u16;
                let arg2 = memory[address.wrapping_add(1) as u16] as u16 * 256;
                Some(arg1 + arg2 + cpu.y as u16)
            }
            _ => { return None; }
        }
    }
}



#[derive(Copy, Clone)]
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

        return data;
    }
}







