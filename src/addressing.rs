// How to interpret the bytes following an instruction.
// Different address modes might address the CPU, memory, or treat the bytes
// as a raw value.
#[derive(Copy, Clone, PartialEq)]
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

// Determine if the address above the input address is in another page.
// For instance, the address 0x03FF and 0x0400 are in different pages.
// The check boils down to checking if the two LSB are FF.
pub fn address_crosses_page(address: u16) -> bool {
    (address & 0xFF) == 0xFF
}

