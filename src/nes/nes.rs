use crate::addressing::AddressingMode;
use crate::instruction::Instruction;
use crate::cpu::{CPUFlags};
use super::NES;

impl NES {
    // Wrapper for branch instructions. 
    // Returns the extra cycles incurred by the instruction:
    // 1 extra if branch taken, and 1 extra if it crosses a page
    fn branch_if(&mut self, condition: bool, addr: Option<u16>) {
        if condition {
            let offset = addr.unwrap() as i8;
            let target = self.cpu.pc.wrapping_add(2).wrapping_add(offset as i16 as u16);

            let crossed_page = (self.cpu.pc & 0xFF00) != (target & 0xFF00);
            self.cpu.pc = target;

            self.cycles += 1 + crossed_page as usize;
        }
    }

    fn compare_reg(&mut self, mode: AddressingMode, addr: Option<u16>, register: u8) {
        let (value, page_crossed) = self.resolve_value_from_addressmode(mode, addr);
        self.cpu.set_flag(CPUFlags::CARRY, register >= value);
        self.cpu.set_flag(CPUFlags::ZERO, register == value);
        self.cpu.set_flag(CPUFlags::NEGATIVE, (value & 0x80) != 0);

        if page_crossed { self.cycles += 1; }
    }

    fn shift_rmw(&mut self, mode: AddressingMode, addr: Option<u16>, op: impl Fn(u8, bool) -> (u8, bool)) {
        let (value, _) = self.resolve_value_from_addressmode(mode, addr);
        let (result, carry) = op(value, self.cpu.get_flag(CPUFlags::CARRY));
        self.cpu.set_flag(CPUFlags::CARRY, carry);
        self.cpu.set_zn(result);
        match mode {
            AddressingMode::Accumulator => { self.cpu.acc = result; }
            _ => {
                let (address, _) = self.resolve_address(mode, addr.unwrap())
                    .expect("Invalid address mode");
                self.write_addr(address, result);
            }
        }
    }

    // Used by AND, ORA and EOR
    fn bit_operation(&mut self, mode: AddressingMode, arg: Option<u16>,
        op: impl Fn(u8, u8) -> u8) {

        let (memory, page_crossed) = self.resolve_value_from_addressmode(mode, arg);

        self.cpu.acc = op(self.cpu.acc, memory);
        self.cpu.set_zn(self.cpu.acc);

        if page_crossed { self.cycles += 1; }
    }

    // Push result onto the stack. The most common way of doing so, used by e.g. PHA
    fn stack_push(&mut self, value: u8) {
        self.write_addr(0x100 + self.cpu.s as u16, value);
        self.cpu.s = self.cpu.s.wrapping_sub(1);
    }

    fn stack_pull(&mut self) -> u8 {
        self.cpu.s = self.cpu.s.wrapping_add(1);
        let result = self.read(0x100 + self.cpu.s as u16);
        self.cpu.set_zn(result);
        return result;
    }



    pub fn tick(&mut self) {
        let opcode = self.read(self.cpu.pc);
        let instruction_data = self.instruction_data[opcode as usize];
        let addr_mode = instruction_data.address_mode;
        self.cycles = instruction_data.cycles as usize;
        
        let arg: Option<u16> = match instruction_data.bytes {
            1 => None,
            2 => Some(self.read(self.cpu.pc.wrapping_add(1)) as u16),
            3 => Some(((self.read(self.cpu.pc.wrapping_add(1)) as u16) >> 8) + (self.read(self.cpu.pc.wrapping_add(2)) as u16)),
            _ => panic!("Invalid number of bytes for opcode.")
        };


        // execute instruction...
        match instruction_data.instruction {
            Instruction::ADC => {
                let (memory, page_crossed) = self.resolve_value_from_addressmode(addr_mode, arg);

                // A = A + memory + C
                // Detect overflow to set the carry bit. Bit hacky, but merge two overflowing_add
                // operations
                let (result, wrapped_a) = self.cpu.acc.overflowing_add(memory);
                let (result, wrapped_b) = result.overflowing_add(self.cpu.flag_as_u8(CPUFlags::CARRY));
                let wrapped = wrapped_a | wrapped_b;

                self.cpu.set_flag(CPUFlags::CARRY, wrapped);
                self.cpu.set_flag(CPUFlags::OVERFLOW, ((result ^ self.cpu.acc) & (result ^ memory) & 0x80) != 0);
                self.cpu.set_zn(result);

                self.cpu.acc = result;

                if page_crossed { self.cycles += 1; }
                
            }
            Instruction::SBC => {
                // A = A - memory - ~C, or A = A + ~memory + C
                let (memory, page_crossed) = self.resolve_value_from_addressmode(addr_mode, arg);

                // First, treat as u16 to detect overflow
                let result: u16 = self.cpu.acc as u16 + (!memory as u16) + (self.cpu.flag_as_u8(CPUFlags::CARRY) as u16);
                self.cpu.set_flag(CPUFlags::CARRY, result > 0xFF);
                // Then, cap to u8 
                let result = result as u8;
                self.cpu.set_flag(CPUFlags::OVERFLOW, ((result ^ self.cpu.acc) & (result ^ !memory) & 0x80) != 0);
                self.cpu.set_zn(result);

                self.cpu.acc = result;
                if page_crossed { self.cycles += 1; }
            }
            Instruction::ASL => self.shift_rmw(addr_mode, arg, |v, _| (v << 1, (v & 0x80) != 0)),
            Instruction::LSR => self.shift_rmw(addr_mode, arg, |v, _| (v >> 1, (v & 0x01) != 0)),
            Instruction::ROL => self.shift_rmw(addr_mode, arg, |v, c| ((v << 1) | c as u8,        (v & 0x80) != 0)),
            Instruction::ROR => self.shift_rmw(addr_mode, arg, |v, c| ((v >> 1) | (c as u8) << 7, (v & 0x01) != 0)),
            Instruction::BCC => { self.branch_if(self.cpu.get_flag(CPUFlags::CARRY), arg); },
            Instruction::BEQ => { self.branch_if(self.cpu.get_flag(CPUFlags::ZERO), arg); },
            Instruction::BMI => { self.branch_if(self.cpu.get_flag(CPUFlags::NEGATIVE), arg); },
            Instruction::BNE => { self.branch_if(!self.cpu.get_flag(CPUFlags::ZERO), arg); },
            Instruction::BPL => { self.branch_if(!self.cpu.get_flag(CPUFlags::NEGATIVE), arg); },
            Instruction::BVC => { self.branch_if(!self.cpu.get_flag(CPUFlags::OVERFLOW), arg); },
            Instruction::BVS => { self.branch_if(self.cpu.get_flag(CPUFlags::OVERFLOW), arg); },
            Instruction::CLC => { self.cpu.set_flag(CPUFlags::CARRY, false); },
            Instruction::CLD => { self.cpu.set_flag(CPUFlags::DECIMAL, false); },
            Instruction::CLI => { self.cpu.set_flag(CPUFlags::IRQ, false); },
            Instruction::CLV => { self.cpu.set_flag(CPUFlags::OVERFLOW, false); },
            Instruction::SEC => { self.cpu.set_flag(CPUFlags::CARRY, true); },
            Instruction::SED => { self.cpu.set_flag(CPUFlags::DECIMAL, true); },
            Instruction::SEI => { self.cpu.set_flag(CPUFlags::IRQ, true); },
            Instruction::CMP => { self.compare_reg(addr_mode, arg, self.cpu.acc); }
            Instruction::CPX => { self.compare_reg(addr_mode, arg, self.cpu.x); }
            Instruction::CPY => { self.compare_reg(addr_mode, arg, self.cpu.y); }
            Instruction::DEC => { 
                let (addr, page_crossed) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                let result = self.read(addr).wrapping_sub(1);
                self.write_addr(addr, result);
                self.cpu.set_zn(result);
            }
            Instruction::DEX => {
                self.cpu.x = self.cpu.x.wrapping_sub(1);
                self.cpu.set_zn(self.cpu.x);
            }
            Instruction::DEY => {
                self.cpu.y = self.cpu.y.wrapping_sub(1);
                self.cpu.set_zn(self.cpu.y);
            }
            Instruction::INC => { 
                let (addr, page_crossed) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                let result = self.read(addr).wrapping_add(1);
                self.write_addr(addr, result);

                self.cpu.set_zn(result);
            }
            Instruction::INX => {
                self.cpu.x = self.cpu.x.wrapping_add(1);
                self.cpu.set_zn(self.cpu.x);
            }
            Instruction::INY => {
                self.cpu.y = self.cpu.y.wrapping_add(1);
                self.cpu.set_zn(self.cpu.y);
            }
            Instruction::PLA => {
                self.cpu.acc = self.stack_pull();
            }
            Instruction::PHA => {
                self.stack_push(self.cpu.acc);
            }
            Instruction::TSX => {
                self.cpu.x = self.cpu.s;
                self.cpu.set_zn(self.cpu.x);
            }
            Instruction::TXS => {
                self.cpu.s = self.cpu.x
            }
            Instruction::TAX => {
                self.cpu.x = self.cpu.acc;
                self.cpu.set_zn(self.cpu.x);
            }
            Instruction::TXA => {
                self.cpu.acc = self.cpu.x;
                self.cpu.set_zn(self.cpu.x);
            }
            Instruction::TAY => {
                self.cpu.y = self.cpu.acc;
                self.cpu.set_zn(self.cpu.y);
            }
            Instruction::TYA => {
                self.cpu.acc = self.cpu.y;
                self.cpu.set_zn(self.cpu.y);
            }
            Instruction::PHP => {
                self.write_addr(0x100 + (self.cpu.s as u16), self.cpu.flags.bits() & (1 << 4) & (1 << 5));
                self.cpu.s = self.cpu.s.wrapping_sub(1);
            }
            Instruction::PLP => {
                self.cpu.s = self.cpu.s.wrapping_add(1);
                let bits = self.read(0x100 + (self.cpu.s as u16));
                self.cpu.set_flag(CPUFlags::CARRY,      bits & (1 << 0) != 0);
                self.cpu.set_flag(CPUFlags::ZERO,       bits & (1 << 1) != 0);
                self.cpu.set_flag(CPUFlags::IRQ,        bits & (1 << 2) != 0);
                self.cpu.set_flag(CPUFlags::DECIMAL,    bits & (1 << 3) != 0);
                self.cpu.set_flag(CPUFlags::OVERFLOW,   bits & (1 << 6) != 0);
                self.cpu.set_flag(CPUFlags::NEGATIVE,   bits & (1 << 7) != 0);
            }
            Instruction::LDA => { 
                let (addr, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                self.cpu.acc = self.read(addr); self.cpu.set_zn(self.cpu.acc); 
            }
            Instruction::LDX => { 
                let (addr, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                self.cpu.x = self.read(addr); self.cpu.set_zn(self.cpu.x); 
            }
            Instruction::LDY => { 
                let (addr, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                self.cpu.y = self.read(addr); self.cpu.set_zn(self.cpu.y); 
            }
            Instruction::STA => {
                let (addr, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                self.write_addr(addr, self.cpu.acc);
            }
            Instruction::STX => {
                let (addr, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                self.write_addr(addr, self.cpu.x);
            }
            Instruction::STY => {
                let (addr, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                self.write_addr(addr, self.cpu.y);
            }
            Instruction::NOP => {
            }
            Instruction::JMP => {
                // Begin to construct 16-bit value to be stored in PC.
                // Denote the two component bytes as highbyte and lowbyte
                let (highbyte_address, _) = self.resolve_address(addr_mode, arg.unwrap()).unwrap();
                let high_byte = self.read(highbyte_address);

                // Emulate NES CPU bug. When addressing a 16-bit value (spanning two byte memory
                // addresses) that crosses a page, the low byte is read from the wrong address.
                // Specifically, the low byte is read from the 0th address of the first page.
                // For example, reading from high byte 0x03FF will read form low byte 0x0300, not
                // 0x0400.
                let lowbyte_address = 
                    if addr_mode == AddressingMode::Indirect && crate::addressing::address_crosses_page(highbyte_address) {
                        highbyte_address & 0xFF00
                    }  else {
                        highbyte_address.wrapping_add(1)
                    };

                let low_byte = self.read(lowbyte_address);
                self.cpu.pc = ((high_byte as u16) << 8) & (low_byte as u16);
            }
            Instruction::JSP => {
                self.stack_push(((self.cpu.pc + 2) >> 8) as u8); // Push high byte
                self.stack_push((self.cpu.pc + 2) as u8); // Push low byte

                let addr = arg.unwrap();
                self.cpu.pc = ((self.read(addr) as u16) << 8) & (self.read(addr.wrapping_add(1)) as u16);
            }
            Instruction::RTS => {
                let low = self.stack_pull();
                let high = self.stack_pull();
                self.cpu.pc = (((high as u16) << 8) & (low as u16)).wrapping_add(1);
            }
            Instruction::BRK => {
                let value = self.cpu.pc.wrapping_add(2);
                self.stack_push((value >> 8) as u8); // push high byte
                self.stack_push(value as u8); // push low byte
                self.stack_push(self.cpu.flags.bits() | (1 << 5) | (1 << 4));
                self.cpu.pc = 0xFFFE;
            }
            Instruction::RTI => {
               let flags = self.stack_pull();
               self.cpu.pc = ((self.stack_pull() as u16) << 8) & (self.stack_pull() as u16);

               self.cpu.set_flag(CPUFlags::CARRY,       (flags & (1 << 0)) != 0);
               self.cpu.set_flag(CPUFlags::ZERO,        (flags & (1 << 1)) != 0);
               self.cpu.set_flag(CPUFlags::IRQ,         (flags & (1 << 2)) != 0);
               self.cpu.set_flag(CPUFlags::DECIMAL,     (flags & (1 << 3)) != 0);
               self.cpu.set_flag(CPUFlags::OVERFLOW,    (flags & (1 << 6)) != 0);
               self.cpu.set_flag(CPUFlags::NEGATIVE,    (flags & (1 << 7)) != 0);
            }
            Instruction::AND => { self.bit_operation(addr_mode, arg, |x, y| x & y) }
            Instruction::ORA => { self.bit_operation(addr_mode, arg, |x, y| x | y) }
            Instruction::EOR => { self.bit_operation(addr_mode, arg, |x, y| x ^ y) }
            Instruction::BIT => {
                let (value, _) = self.resolve_value_from_addressmode(addr_mode, arg);
                self.cpu.set_flag(CPUFlags::OVERFLOW, (value & 0x40) != 0);
                self.cpu.set_zn(value);
            }
            _ => panic!("unimplemented instruction {}", instruction_data.instruction)
        }

        // advance program counter
        self.cpu.pc = self.cpu.pc.wrapping_add(instruction_data.bytes as u16);

    }
}
