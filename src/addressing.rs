// How to interpret the bytes following an instruction.
// Different address modes might address the CPU, memory, or treat the bytes
// as a raw value.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AddressingMode {
    Immediate,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect, // Used by JMP
    IndirectX,
    IndirectY,
    Accumulator,
    Relative
}

// True when adding an index moved the effective address into a different
// 256-byte page (i.e. the high byte changed). Used to charge the extra cycle
// for AbsoluteX/AbsoluteY/IndirectY reads.
pub fn address_crosses_page(base: u16, effective: u16) -> bool {
    (base & 0xFF00) != (effective & 0xFF00)
}

