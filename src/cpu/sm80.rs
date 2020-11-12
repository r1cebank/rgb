use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

use super::cycles::{CB_CYCLES, OP_CYCLES};
use super::instruction::{
    AddressLocation,
    Address, Condition, Instruction, OperationType, Register, SourceType, TargetType, Value,
};
use super::registers::{Flag, Registers};
use crate::cpu::instruction::OperationType::RegisterToRegister;

pub struct Core {
    pub memory: Rc<RefCell<dyn Memory>>,
    pub registers: Registers,
    halted: bool,
    ei: bool,
}

enum DataType {
    U8(u8),
    U16(u16),
}

impl Core {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Core {
        Self {
            memory,
            registers: Registers::new(),
            ei: true,
            halted: false,
        }
    }
    /// When not boot rom is supplied, we call this to make sure the following state is set
    pub fn simulate_boot_rom(&mut self) {
        self.registers.a = 0x01;
        self.registers.f = 0xb0;
        self.registers.b = 0x00;
        self.registers.c = 0x13;
        self.registers.d = 0x00;
        self.registers.e = 0xd8;
        self.registers.h = 0x01;
        self.registers.l = 0x4d;
        self.registers.pc = 0x0100;
        self.registers.sp = 0xfffe;
    }
    /// Get the next byte in the memory location
    fn get_next(&mut self) -> u8 {
        let value = self.memory.borrow().get(self.registers.pc);
        self.registers.pc += 1;
        value
    }
    /// Get the next work in the next memory location
    fn get_next_word(&mut self) -> u16 {
        let value = self.memory.borrow().get_word(self.registers.pc);
        self.registers.pc += 2;
        value
    }
    /// Push value to the stack and update the stack pointer
    fn stack_push(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.memory.borrow_mut().set_word(self.registers.sp, value);
    }
    /// Pop the current value on the stack
    fn stack_pop(&mut self) -> u16 {
        let value = self.memory.borrow_mut().get_word(self.registers.sp);
        self.registers.sp += 2;
        value
    }
    fn set_register_16(&mut self, register: Register, value: u16) {
        match register {
            Register::BC => {
                self.registers.set_bc(value);
            }
            Register::DE => {
                self.registers.set_de(value);
            }
            Register::HL => {
                self.registers.set_hl(value);
            }
            Register::AF => {
                self.registers.set_af(value);
            }
            Register::SP => {
                self.registers.sp = value;
            }
            _ => {
                panic!("Invalid assignment to register {}", register);
            }
        }
    }
    fn set_register(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.registers.a = value,
            Register::B => self.registers.b = value,
            Register::C => self.registers.c = value,
            Register::D => self.registers.d = value,
            Register::E => self.registers.e = value,
            Register::H => self.registers.h = value,
            Register::L => self.registers.l = value,
            _ => {
                panic!("Invalid assignment to register");
            }
        }
    }
    fn get_register(&self, register: Register) -> DataType {
        match register {
            Register::A => DataType::U8(self.registers.a),
            Register::B => DataType::U8(self.registers.b),
            Register::C => DataType::U8(self.registers.c),
            Register::D => DataType::U8(self.registers.d),
            Register::E => DataType::U8(self.registers.e),
            Register::H => DataType::U8(self.registers.h),
            Register::L => DataType::U8(self.registers.l),
            Register::BC => DataType::U16(self.registers.get_bc()),
            Register::DE => DataType::U16(self.registers.get_de()),
            Register::HL => DataType::U16(self.registers.get_hl()),
            Register::AF => DataType::U16(self.registers.get_af()),
            Register::SP => DataType::U16(self.registers.sp),
            _ => {
                panic!("Invalid assignment to register");
            }
        }
    }
    fn set_address_value(&mut self, address: Address, value: u8) {
        match address {
            Address::A8 => {
                let address = 0xff00 | u16::from(self.get_next());
                self.memory.borrow_mut().set(address, value);
            }
            Address::A16 => {
                let address = self.get_next_word();
                self.memory.borrow_mut().set(address, value);
            }
            Address::C => {
                self.memory
                    .borrow_mut()
                    .set(0xff00 | u16::from(self.registers.c), value);
            }
            Address::BC => {
                self.memory.borrow_mut().set(self.registers.get_bc(), value);
            }
            Address::DE => {
                self.memory.borrow_mut().set(self.registers.get_de(), value);
            }
            Address::HL => {
                self.memory.borrow_mut().set(self.registers.get_hl(), value);
            }
            Address::A16 => {
                let address = self.get_next_word();
                self.memory.borrow_mut().set(address, value);
            }
            Address::HLP => {
                let address = self.registers.get_hl();
                self.memory.borrow_mut().set(address, value);
                self.registers.set_hl(address + 1);
            }
            Address::HLM => {
                let address = self.registers.get_hl();
                self.memory.borrow_mut().set(address, value);
                self.registers.set_hl(address - 1);
            }
            _ => {
                panic!("Not Implemented");
            }
        }
    }
    // Complement carry flag. If C flag is set, then reset it. If C flag is reset, then set it.
    fn alu_ccf(&mut self) {
        let carry = !self.registers.get_flag(Flag::C);
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
    }
    // Set Carry flag.
    fn alu_scf(&mut self) {
        self.registers.set_flag(Flag::C, true);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
    }
    // Complement A register. (Flip all bits.)
    fn alu_cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::N, true);
    }
    // Add n to current address and jump to it.
    // n = one byte signed immediate value
    fn alu_jr(&mut self, n: u8) {
        let n = n as i8;
        self.registers.pc = ((u32::from(self.registers.pc) as i32) + i32::from(n)) as u16;
    }
    // Rotate n right through Carry flag.
    fn alu_rr(&mut self, n: u8) -> u8 {
        let carry = n & 0x01 == 0x01;
        let result = if self.registers.get_flag(Flag::C) {
            0x80 | (n >> 1)
        } else {
            n >> 1
        };
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Decimal adjust register A. This instruction adjusts register A so that the correct representation of Binary
    // Coded Decimal (BCD) is obtained.
    fn alu_daa(&mut self) {
        let mut a = self.registers.a;
        let mut adjust = if self.registers.get_flag(Flag::C) {
            0x60
        } else {
            0x00
        };
        if self.registers.get_flag(Flag::H) {
            adjust |= 0x06;
        };
        if !self.registers.get_flag(Flag::N) {
            if a & 0x0f > 0x09 {
                adjust |= 0x06;
            };
            if a > 0x99 {
                adjust |= 0x60;
            };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }
        self.registers.set_flag(Flag::C, adjust >= 0x60);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::Z, a == 0x00);
        self.registers.a = a;
    }
    // Rotate n right. Old bit 0 to Carry flag.
    fn alu_rrc(&mut self, n: u8) -> u8 {
        let carry = n & 0x01 == 0x01;
        let result = if carry { 0x80 | (n >> 1) } else { n >> 1 };
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Rotate n left through Carry flag.
    fn alu_rl(&mut self, n: u8) -> u8 {
        let carry = (n & 0x80) >> 7 == 0x01;
        let result = (n << 1) + u8::from(self.registers.get_flag(Flag::C));
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Rotate n left. Old bit 7 to Carry flag.
    fn alu_rlc(&mut self, n: u8) -> u8 {
        let carry = (n & 0x80) >> 7 == 0x01;
        let result = (n << 1) | u8::from(carry);
        self.registers.set_flag(Flag::C, carry);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Decrement number
    fn alu_dec(&mut self, n: u8) -> u8 {
        let result = n.wrapping_sub(1);
        self.registers.set_flag(Flag::H, n.trailing_zeros() >= 4);
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::Z, result == 0);
        result
    }
    // Increment number
    fn alu_inc(&mut self, n: u8) -> u8 {
        let result = n.wrapping_add(1);
        self.registers.set_flag(Flag::H, (n & 0x0f) + 0x01 > 0x0f);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        result
    }
    // Subtract n from A.
    fn alu_sub(&mut self, n: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(n);
        self.registers
            .set_flag(Flag::C, u16::from(a) < u16::from(n));
        self.registers.set_flag(Flag::H, (a & 0x0f) < (n & 0x0f));
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Subtract n + Carry flag from A.
    fn alu_sbc(&mut self, n: u8) {
        let a = self.registers.a;
        let carry = u8::from(self.registers.get_flag(Flag::C));
        let result = a.wrapping_sub(n).wrapping_sub(carry);
        self.registers
            .set_flag(Flag::C, u16::from(a) < u16::from(n) + u16::from(carry));
        self.registers
            .set_flag(Flag::H, (a & 0x0f) < (n & 0x0f) + carry);
        self.registers.set_flag(Flag::N, true);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Add n + Carry flag to A.
    fn alu_adc(&mut self, n: u8) {
        let a = self.registers.a;
        let carry = u8::from(self.registers.get_flag(Flag::C));
        let result = a.wrapping_add(n).wrapping_add(carry);
        self.registers.set_flag(
            Flag::C,
            u16::from(a) + u16::from(n) + u16::from(carry) > 0xff,
        );
        self.registers
            .set_flag(Flag::H, (a & 0x0f) + (n & 0x0f) + (carry & 0x0f) > 0x0f);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Add n to A.
    fn alu_add(&mut self, n: u8) {
        let a = self.registers.a;
        let result = a.wrapping_add(n);
        self.registers
            .set_flag(Flag::C, u16::from(a) + u16::from(n) > 0xff);
        self.registers
            .set_flag(Flag::H, (a & 0x0f) + (n & 0x0f) > 0x0f);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Add n to HL
    fn alu_add_hl(&mut self, n: u16) {
        let value = self.registers.get_hl();
        let result = value.wrapping_add(n);
        self.registers.set_flag(Flag::C, value > 0xffff - n);
        self.registers
            .set_flag(Flag::H, (value & 0x0fff) + (n & 0x0fff) > 0x0fff);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_hl(result);
    }
    // Logically AND n with A, result in A.
    fn alu_and(&mut self, n: u8) {
        let result = self.registers.a & n;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Logical OR n with register A, result in A.
    fn alu_or(&mut self, n: u8) {
        let result = self.registers.a | n;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Logical exclusive OR n with register A, result in A.
    fn alu_xor(&mut self, n: u8) {
        let result = self.registers.a ^ n;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    // Compare A with n. This is basically an A - n subtraction instruction but the results are thrown away.
    fn alu_cp(&mut self, n: u8) {
        let r = self.registers.a;
        self.alu_sub(n);
        self.registers.a = r;
    }
    fn set_address_value_16(&mut self, address: Address, value: u16) {
        match address {
            Address::A16 => {
                let address = self.get_next_word();
                self.memory.borrow_mut().set_word(address, value);
            }
            _ => {
                panic!("Not Implemented");
            }
        }
    }
    fn register_to_address(&mut self, address: Address, register: Register) {
        let register_value = self.get_register(register);
        match address {
            Address::HLM => {
                trace!("LD (HL-), {}", register);
            }
            Address::HLP => {
                trace!("LD (HL+), {}", register);
            }
            _ => {
                trace!("LD ({}), {}", address, register);
            }
        }
        match register_value {
            DataType::U8(value) => {
                self.set_address_value(address, value);
            }
            DataType::U16(value) => {
                self.set_address_value_16(address, value);
            }
        }
    }
    fn get_condition(&self, condition: Condition) -> bool {
        let is_condition = match condition {
            Condition::NotZero => !self.registers.get_flag(Flag::Z),
            Condition::Zero => self.registers.get_flag(Flag::Z),
            Condition::Carry => self.registers.get_flag(Flag::C),
            Condition::NotCarry => !self.registers.get_flag(Flag::C),
            Condition::Always => true,
        };
        is_condition
    }
    fn get_address(&mut self, address: Address) -> u8 {
        match address {
            Address::A8 => {
                let address = 0xff00 | u16::from(self.get_next());
                self.memory.borrow().get(address)
            }
            Address::C => self
                .memory
                .borrow()
                .get(0xff00 | u16::from(self.registers.c)),
            Address::BC => self.memory.borrow().get(self.registers.get_bc()),
            Address::DE => self.memory.borrow().get(self.registers.get_de()),
            Address::A16 => {
                let address = self.get_next_word();
                self.memory.borrow().get(address)
            }
            Address::HL => self.memory.borrow().get(self.registers.get_hl()),
            Address::HLP => {
                let address = self.registers.get_hl();
                let value = self.memory.borrow_mut().get(address);
                self.registers.set_hl(address + 1);
                value
            }
            Address::HLM => {
                let address = self.registers.get_hl();
                let value = self.memory.borrow_mut().get(address);
                self.registers.set_hl(address - 1);
                value
            }
            _ => {
                panic!("Not Implemented");
            }
        }
    }
    fn value_to_address(&mut self, address: Address, value: Value) {
        match value {
            Value::D8 => {
                let d8 = self.get_next();
                trace!("LD ({}), ${:04x}", address, d8);
                // Set the register value
                self.set_address_value(address, d8);
            }
            _ => {
                panic!("Not implemented: ({:?}) <- {:?}", address, value);
            }
        }
    }
    fn value_to_register(&mut self, register: Register, value: Value) {
        match value {
            Value::D16 => {
                let d16 = self.get_next_word();
                trace!("LD {}, ${:04x}", register, d16);

                // Set the register value
                self.set_register_16(register, d16);
            }
            Value::D8 => {
                let d8 = self.get_next();
                trace!("LD {}, ${:04x}", register, d8);
                // Set the register value
                self.set_register(register, d8);
            }
            _ => {
                panic!("Not implemented: {:?} <- {:?}", register, value);
            }
        }
    }
    fn address_to_register(&mut self, register: Register, address: Address) {
        let address_value = self.get_address(address);
        match address {
            Address::HLM => {
                trace!("LD {}, (HL-)", register);
            }
            Address::HLP => {
                trace!("LD {}, (HL+)", register);
            }
            _ => {
                trace!("LD {}, ({})", register, address);
            }
        }
        self.set_register(register, address_value);
    }
    fn get_cycle(&self, opcode: u8, is_prefixed: bool) -> u32 {
        let exception_cycle = match opcode {
            0x20 | 0x30 => {
                if self.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x01
                }
            }
            0x28 | 0x38 => {
                if self.registers.get_flag(Flag::Z) {
                    0x01
                } else {
                    0x00
                }
            }
            0xc0 | 0xd0 => {
                if self.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x03
                }
            }
            0xc8 | 0xcc | 0xd8 | 0xdc => {
                if self.registers.get_flag(Flag::Z) {
                    0x03
                } else {
                    0x00
                }
            }
            0xc2 | 0xd2 => {
                if self.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x01
                }
            }
            0xca | 0xda => {
                if self.registers.get_flag(Flag::Z) {
                    0x01
                } else {
                    0x00
                }
            }
            0xc4 | 0xd4 => {
                if self.registers.get_flag(Flag::Z) {
                    0x00
                } else {
                    0x03
                }
            }
            _ => 0x00,
        };
        if is_prefixed {
            CB_CYCLES[opcode as usize]
        } else {
            OP_CYCLES[opcode as usize] + exception_cycle
        }
    }
    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NAI => {
                panic!("Not supposed to run the NAI instruction");
            }
            Instruction::NOP => {
                trace!("NOP");
            } // Not doing anything in NOP
            Instruction::LD(operation_type) => {
                // Finished ✔
                match operation_type {
                    OperationType::ValueToRegister(register, value) => {
                        self.value_to_register(register, value)
                    }
                    OperationType::RegisterToAddress(address, register) => {
                        self.register_to_address(address, register);
                    }
                    OperationType::AddressToRegister(register, address) => {
                        self.address_to_register(register, address);
                    }
                    OperationType::ValueToAddress(address, value) => {
                        self.value_to_address(address, value);
                    }
                    OperationType::RegisterToRegister(target, source) => {
                        let register_value = self.get_register(source);
                        trace!("LD {}, {}", target, source);
                        match register_value {
                            DataType::U8(value) => {
                                self.set_register(target, value);
                            }
                            _ => {
                                panic!("Invalid datatype u16 for LD");
                            }
                        }
                    }
                }
            }
            Instruction::LDH(operation_type) => {
                // Finished x
                match operation_type {
                    OperationType::RegisterToAddress(address, register) => {
                        trace!("LDH ({}), {}", address, register);
                        let address = 0xff00 | u16::from(self.get_next());
                        self.memory.borrow_mut().set(address, self.registers.a);
                    }
                    OperationType::AddressToRegister(register, address) => {
                        trace!("LDH {}, ({})", register, address);
                        let address = 0xff00 | self.get_next_word();
                        self.registers.a = self.memory.borrow().get(address);
                    }
                    _ => {
                        panic!("Invalid operation type {} for LDH", operation_type);
                    }
                }
            }
            Instruction::INC(target_type) => {
                // Finished ✔
                match target_type {
                    TargetType::Register(register) => {
                        trace!("INC {}", register);
                        let register_value = self.get_register(register);
                        match register_value {
                            DataType::U8(value) => {
                                let result = self.alu_inc(value);
                                self.set_register(register, result);
                            }
                            DataType::U16(value) => {
                                let value = value.wrapping_add(1);
                                self.set_register_16(register, value);
                            }
                        }
                    }
                    TargetType::Address(address) => {
                        trace!("INC ({})", address);
                        match address {
                            Address::HL => {
                                let address = self.registers.get_hl();
                                let value = self.memory.borrow().get(address);
                                let result = self.alu_inc(value);
                                self.memory.borrow_mut().set(address, result);
                            }
                            _ => {
                                panic!("Invalid address {} for INC", address);
                            }
                        }
                    }
                }
            }
            Instruction::DEC(target_type) => {
                // Finished ✔
                match target_type {
                    TargetType::Register(register) => {
                        trace!("DEC {}", register);
                        let register_value = self.get_register(register);
                        match register_value {
                            DataType::U8(value) => {
                                let result = self.alu_dec(value);
                                self.set_register(register, result);
                            }
                            DataType::U16(value) => {
                                let value = value.wrapping_sub(1);
                                self.set_register_16(register, value);
                            }
                        }
                    }
                    TargetType::Address(address) => {
                        trace!("DEC ({})", address);
                        match address {
                            Address::HL => {
                                let address = self.registers.get_hl();
                                let value = self.memory.borrow().get(address);
                                let result = self.alu_dec(value);
                                self.memory.borrow_mut().set(address, result);
                            }
                            _ => {
                                panic!("Invalid address {} for INC", address);
                            }
                        }
                    }
                }
            }
            Instruction::AND(source_type) => {
                // Finished ✔
                match source_type {
                    SourceType::Register(source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("AND A, {}", source);
                                self.alu_and(value);
                            }
                            _ => {
                                panic!("Invalid datatype u16 for AND");
                            }
                        }
                    }
                    SourceType::Address(address) => {
                        trace!("AND A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_and(address_value);
                    }
                    SourceType::Value(_) => {
                        trace!("AND A, d8");
                        let value = self.get_next();
                        self.alu_and(value);
                    }
                }
            }
            Instruction::XOR(source_type) => {
                // Finished ✔
                match source_type {
                    SourceType::Register(source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("XOR A, {}", source);
                                self.alu_xor(value);
                            }
                            _ => {
                                panic!("Invalid datatype u16 for XOR");
                            }
                        }
                    }
                    SourceType::Address(address) => {
                        trace!("XOR A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_xor(address_value);
                    }
                    SourceType::Value(_) => {
                        trace!("XOR A, d8");
                        let value = self.get_next();
                        self.alu_xor(value);
                    }
                }
            }
            Instruction::OR(source_type) => {
                // Finished ✔
                match source_type {
                    SourceType::Register(source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("OR A, {}", source);
                                self.alu_or(value);
                            }
                            _ => {
                                panic!("Invalid datatype u16 for OR");
                            }
                        }
                    }
                    SourceType::Address(address) => {
                        trace!("OR A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_or(address_value);
                    }
                    SourceType::Value(_) => {
                        trace!("OR A, d8");
                        let value = self.get_next();
                        self.alu_or(value);
                    }
                }
            }
            Instruction::CP(source_type) => {
                // Finished ✔
                match source_type {
                    SourceType::Register(source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("CP {}", source);
                                self.alu_cp(value);
                            }
                            _ => {
                                panic!("Invalid datatype u16 for OR");
                            }
                        }
                    }
                    SourceType::Address(address) => {
                        trace!("CP ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_cp(address_value);
                    }
                    SourceType::Value(_) => {
                        trace!("CP d8");
                        let value = self.get_next();
                        self.alu_cp(value);
                    }
                }
            }
            Instruction::RET(condition) => {
                // Finished ✔
                trace!("RET {}", condition);
                let can_return = self.get_condition(condition);
                if can_return {
                    self.registers.pc = self.stack_pop();
                }
            }
            Instruction::RETI => {
                trace!("RETI");
                self.registers.pc = self.stack_pop();
                self.ei = true;
            }
            Instruction::POP(register) => {
                // Finished ✔
                trace!("POP {}", register);
                let value = self.stack_pop();
                self.set_register_16(register, value);
            }
            Instruction::PUSH(register) => {
                // Finished ✔
                trace!("PUSH {}", register);
                let register_value = self.get_register(register);
                match register_value {
                    DataType::U16(value) => {
                        self.stack_push(value);
                    }
                    _ => {
                        panic!("Invalid datatype u16 for PUSH");
                    }
                }
            }
            Instruction::RLCA => {
                // Finished ✔
                trace!("RLCA");
                self.registers.a = self.alu_rlc(self.registers.a);
                self.registers.set_flag(Flag::Z, false);
            }
            Instruction::RRCA => {
                // Finished ✔
                trace!("RRCA");
                self.registers.a = self.alu_rrc(self.registers.a);
                self.registers.set_flag(Flag::Z, false);
            }
            Instruction::RLA => {
                trace!("RLA");
                self.registers.a = self.alu_rl(self.registers.a);
                self.registers.set_flag(Flag::Z, false);
            }
            Instruction::RRA => {
                trace!("RRA");
                self.registers.a = self.alu_rr(self.registers.a);
                self.registers.set_flag(Flag::Z, false);
            }
            Instruction::DAA => {
                trace!("DAA");
                self.alu_daa();
            }
            Instruction::JR(condition, value) => {
                // Finished ✔
                let can_jump = self.get_condition(condition);
                let address = self.get_next();
                trace!("JR {}, ${:04x}", condition, address);
                if can_jump {
                    self.alu_jr(address);
                }
            }
            Instruction::JP(condition, address) => {
                // Finished ✔
                if address == Address::A16 {
                    let jump_location = self.get_next_word();
                    trace!("JP {}, ${:04x}", condition, jump_location);
                    let can_jump = self.get_condition(condition);
                    if can_jump {
                        self.registers.pc = jump_location;
                    }
                }
                if address == Address::HL {
                    let jump_location = self.registers.get_hl();
                    trace!("JP {}, ${:04x}", condition, jump_location);
                    let can_jump = self.get_condition(condition);
                    if can_jump {
                        self.registers.pc = jump_location;
                    }
                }
            }
            Instruction::CALL(condition, _) => {
                // Finished ✔
                let call_location = self.get_next_word();
                trace!("CALL {}, ${:04x}", condition, call_location);
                let can_call = self.get_condition(condition);
                if can_call {
                    self.stack_push(self.registers.pc);
                    self.registers.pc = call_location;
                }
            }
            Instruction::DI => {
                trace!("DI");
                self.ei = false;
            }
            Instruction::EI => {
                trace!("EI");
                self.ei = true;
            }
            Instruction::CPL => {
                trace!("CPL");
                self.alu_cpl();
            }
            Instruction::SCF => {
                trace!("SCF");
                self.alu_scf();
            }
            Instruction::CCF => {
                trace!("CCF");
                self.alu_ccf();
            }
            Instruction::HALT => {
                trace!("HALT");
                self.halted = true;
            }
            Instruction::STOP => {
                trace!("STOP");
            }
            Instruction::RST(location) => {
                // Finished ✔
                self.stack_push(self.registers.pc);
                self.registers.pc = location as u16;
            }
            Instruction::ADC(operation_type) => {
                // Finished ✔
                match operation_type {
                    OperationType::RegisterToRegister(_, source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("ADC A, {}", source);
                                self.alu_adc(value);
                            }
                            _ => {
                                panic!("Invalid {} for ADC A", source);
                            }
                        }
                    }
                    OperationType::AddressToRegister(_, address) => {
                        trace!("ADC A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_adc(address_value);
                    }
                    OperationType::ValueToRegister(_, value) => {
                        trace!("ADC A, {}", value);
                        let value = self.get_next();
                        self.alu_adc(value);
                    }
                    _ => panic!("Invalid operation {} for ADC A", operation_type),
                }
            }
            Instruction::SUB(operation_type) => {
                // Finished ✔
                match operation_type {
                    OperationType::RegisterToRegister(_, source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("SUB A, {}", source);
                                self.alu_sub(value);
                            }
                            _ => {
                                panic!("Invalid {} for SUB A", source);
                            }
                        }
                    }
                    OperationType::AddressToRegister(_, address) => {
                        trace!("SUB A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_sub(address_value);
                    }
                    _ => {
                        panic!("Invalid operation {} for SUB A", operation_type);
                    }
                }
            }
            Instruction::SBC(operation_type) => {
                // Finished ✔
                match operation_type {
                    OperationType::RegisterToRegister(_, source) => {
                        let register_value = self.get_register(source);
                        match register_value {
                            DataType::U8(value) => {
                                trace!("SBC A, {}", source);
                                self.alu_sbc(value);
                            }
                            _ => {
                                panic!("Invalid {} for SBC A", source);
                            }
                        }
                    }
                    OperationType::AddressToRegister(_, address) => {
                        trace!("SBC A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_sbc(address_value);
                    }
                    OperationType::ValueToRegister(_, value) => {
                        trace!("SBC A, {}", value);
                        let value = self.get_next();
                        self.alu_sbc(value);
                    }
                    _ => {
                        panic!("Invalid operation {} for SBC A", operation_type);
                    }
                }
            }
            Instruction::ADD(operation_type) => {
                // Finished ✔
                match operation_type {
                    OperationType::RegisterToRegister(target, source) => {
                        if target == Register::HL {
                            let register_value = self.get_register(source);
                            match register_value {
                                DataType::U16(value) => {
                                    trace!("ADD HL, {}", source);
                                    self.alu_add_hl(value);
                                }
                                _ => {
                                    panic!("Invalid {} for ADD HL", source);
                                }
                            }
                        }
                        if target == Register::A {
                            let register_value = self.get_register(source);
                            match register_value {
                                DataType::U8(value) => {
                                    trace!("ADD A, {}", source);
                                    self.alu_add(value);
                                }
                                _ => {
                                    panic!("Invalid {} for ADD A", source);
                                }
                            }
                        }
                    }
                    OperationType::AddressToRegister(_, address) => {
                        trace!("ADD A, ({})", address);
                        let address_value = self.get_address(address);
                        self.alu_add(address_value);
                    }
                    OperationType::ValueToRegister(register, _) => {
                        if register == Register::A {
                            trace!("ADD A, d8");
                            let next_byte = self.get_next();
                            self.alu_add(next_byte);
                        }
                        if register == Register::SP {
                            trace!("ADD SP, r8");
                            let sp = self.registers.sp;
                            let b = i16::from(self.get_next() as i8) as u16;
                            self.registers
                                .set_flag(Flag::C, (sp & 0x00ff) + (b & 0x00ff) > 0x00ff);
                            self.registers
                                .set_flag(Flag::H, (sp & 0x000f) + (b & 0x000f) > 0x000f);
                            self.registers.set_flag(Flag::N, false);
                            self.registers.set_flag(Flag::Z, false);
                            self.registers.sp = sp.wrapping_add(b);
                        }
                    }
                    _ => panic!("Invalid operation {} for AND A", operation_type),
                }
            }
            _ => unimplemented!(),
        }
    }
    /// Execute the next instruction
    pub fn execute_next(&mut self) -> u32 {
        // Grab the next memory location and parse it as opcode
        let mut instruction_opcode = self.get_next();
        let is_prefixed = instruction_opcode == 0xcb;
        if is_prefixed {
            instruction_opcode = self.get_next();
        }

        let instruction = Instruction::from_byte(instruction_opcode, is_prefixed);

        debug!(
            "HEX: {:04x} Decoded: {:?} Prefixed: {}",
            instruction_opcode, instruction, is_prefixed
        );

        // Run the instruction
        self.execute_instruction(instruction);

        // Get the machine cycles for this instruction
        self.get_cycle(instruction_opcode, is_prefixed)
    }
    pub fn tick(&mut self) -> u32 {
        // TODO: Handle interrupts here
        let machine_cycle = self.execute_next();

        // Gameboy 1 machine cycle = 4 clock cycle
        machine_cycle * 4
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    use super::*;
    use crate::cpu::opcodes::{INSTRUCTION_MAP, PREFIX_INSTRUCTION_MAP};
    use crate::memory::Memory;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct TestMemory {
        pub memory: [u8; 0xffff],
    }

    impl TestMemory {
        pub fn new() -> TestMemory {
            Self { memory: [0; 0xffff] }
        }
    }

    impl Memory for TestMemory {
        fn get(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }

        fn set(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }
    }

    fn prepare_memory_word(core: &mut Core, address: u16, word: u16) {
        core.memory.borrow_mut().set_word(address, word);
    }

    fn prepare_memory(core: &mut Core, address: u16, value: u8) {
        core.memory.borrow_mut().set(address, value);
    }

    fn get_new_cpu() -> Core {
        let mut cpu = Core::new(Rc::new(RefCell::new(TestMemory::new())));
        cpu
    }

    #[test]
    fn can_correctly_run_ei_instructions() {
        // Instruction::EI
        let mut cpu = get_new_cpu();
        cpu.ei = false;
        cpu.execute_instruction(Instruction::EI);
        assert!(cpu.ei);
    }

    #[test]
    fn can_correctly_run_di_instructions() {
        // Instruction::DI
        let mut cpu = get_new_cpu();
        cpu.ei = true;
        cpu.execute_instruction(Instruction::DI);
        assert!(!cpu.ei);
    }

    #[test]
    fn can_correctly_run_ldh_instructions() {
        // Instruction::LDH(OperationType::AddressToRegister(Register::A, Address::A8))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x03);
        prepare_memory(&mut cpu, 0xff03, 0x1a);
        cpu.execute_instruction(Instruction::LDH(OperationType::AddressToRegister(Register::A, Address::A8)));
        assert_eq!(cpu.registers.a, 0x1a);
        // Instruction::LDH(OperationType::RegisterToAddress(Address::A8, Register::A))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x03;
        prepare_memory(&mut cpu, 0x0000, 0x1c);
        cpu.execute_instruction(Instruction::LDH(OperationType::RegisterToAddress(Address::A8, Register::A)));
        assert_eq!(cpu.memory.borrow().get(0xff1c), 0x03);
    }

    #[test]
    fn can_correctly_run_rst_instructions() {
        // Instruction::RST(AddressLocation::X00H)
        let mut cpu = get_new_cpu();
        cpu.registers.sp = 0x0002;
        cpu.registers.pc = 0x001c;
        cpu.execute_instruction(Instruction::RST(AddressLocation::X00H));
        assert_eq!(cpu.registers.pc, 0x00);
        assert_eq!(cpu.stack_pop(), 0x001c);
    }

    #[test]
    fn can_correctly_run_push_instructions() {
        // Instruction::PUSH(Register::BC)
        let mut cpu = get_new_cpu();
        cpu.registers.sp = 0x0002;
        cpu.registers.set_bc(0x0011);
        cpu.execute_instruction(Instruction::PUSH(Register::BC));
        assert_eq!(cpu.memory.borrow().get_word(0x0000), 0x0011);
        assert_eq!(cpu.registers.sp, 0x0000);
    }

    #[test]
    fn can_correctly_run_call_instructions() {
        // Instruction::CALL(Condition::NotZero, Address::A16)
        let mut cpu = get_new_cpu();
        cpu.registers.pc = 0x0004;
        cpu.registers.sp = 0x0012;
        prepare_memory_word(&mut cpu, 0x0004, 0x001c);
        cpu.execute_instruction(Instruction::CALL(Condition::NotZero, Address::A16));
        assert_eq!(cpu.registers.pc, 0x001c);
        assert_eq!(cpu.registers.sp, 0x0010);
        assert_eq!(cpu.memory.borrow().get_word(0x0010), 0x0006);
        // Instruction::CALL(Condition::NotZero, Address::A16) NO jump
        let mut cpu = get_new_cpu();
        cpu.registers.set_flag(Flag::Z, true);
        cpu.registers.pc = 0x0004;
        cpu.registers.sp = 0x0012;
        cpu.execute_instruction(Instruction::CALL(Condition::NotZero, Address::A16));
        assert_eq!(cpu.registers.pc, 0x0006);
        assert_eq!(cpu.registers.sp, 0x0012);
    }

    #[test]
    fn can_correctly_run_jp_instructions() {
        // Instruction::JP(Condition::NotZero, Address::A16)
        let mut cpu = get_new_cpu();
        prepare_memory_word(&mut cpu, 0x0000, 0x1111);
        cpu.registers.set_flag(Flag::Z, false);
        cpu.execute_instruction(Instruction::JP(Condition::NotZero, Address::A16));
        assert_eq!(cpu.registers.pc, 0x1111);
        // Instruction::JP(Condition::NotZero, Address::A16) no jump
        let mut cpu = get_new_cpu();
        prepare_memory_word(&mut cpu, 0x0000, 0x1111);
        cpu.registers.set_flag(Flag::Z, true);
        cpu.execute_instruction(Instruction::JP(Condition::NotZero, Address::A16));
        assert_eq!(cpu.registers.pc, 0x0002);
        // Instruction::JP(Condition::Always, Address::HL)
        let mut cpu = get_new_cpu();
        cpu.registers.set_hl(0x001c);
        cpu.execute_instruction(Instruction::JP(Condition::Always, Address::HL));
        assert_eq!(cpu.registers.pc, 0x001c);
    }

    #[test]
    fn can_correctly_run_pop_instructions() {
        // Instruction::POP(Register::BC)
        let mut cpu = get_new_cpu();
        cpu.registers.set_flag(Flag::Z, false);
        cpu.registers.sp = 0x0002;
        prepare_memory_word(&mut cpu, 0x0002, 0x0101);
        cpu.execute_instruction(Instruction::POP(Register::BC));
        assert_eq!(cpu.registers.get_bc(), 0x0101);
    }

    #[test]
    fn can_correctly_run_reti_instructions() {
        // Instruction::RETI
        let mut cpu = get_new_cpu();
        cpu.registers.set_flag(Flag::Z, false);
        cpu.registers.sp = 0x0002;
        cpu.ei = false;
        prepare_memory_word(&mut cpu, 0x0002, 0x0101);
        cpu.execute_instruction(Instruction::RETI);
        assert_eq!(cpu.registers.pc, 0x0101);
        assert!(cpu.ei);
    }

    #[test]
    fn can_correctly_run_ret_instructions() {
        // Instruction::RET(Condition::NotZero)
        let mut cpu = get_new_cpu();
        cpu.registers.set_flag(Flag::Z, false);
        cpu.registers.sp = 0x0002;
        prepare_memory_word(&mut cpu, 0x0002, 0x0101);
        cpu.execute_instruction(Instruction::RET(Condition::NotZero));
        assert_eq!(cpu.registers.pc, 0x0101);
        // Instruction::RET(Condition::NotZero) no jump
        let mut cpu = get_new_cpu();
        cpu.registers.set_flag(Flag::Z, true);
        cpu.execute_instruction(Instruction::RET(Condition::NotZero));
        assert_eq!(cpu.registers.pc, 0x0000);
        // Instruction::RET(Condition::Always)
        let mut cpu = get_new_cpu();
        cpu.registers.sp = 0x0002;
        prepare_memory_word(&mut cpu, 0x0002, 0x0101);
        cpu.execute_instruction(Instruction::RET(Condition::Always));
        assert_eq!(cpu.registers.pc, 0x0101);
        // Instruction::RET(Condition::Carry)
        let mut cpu = get_new_cpu();
        cpu.registers.sp = 0x0002;
        cpu.registers.set_flag(Flag::C, true);
        prepare_memory_word(&mut cpu, 0x0002, 0x0101);
        cpu.execute_instruction(Instruction::RET(Condition::Carry));
        assert_eq!(cpu.registers.pc, 0x0101);
        // Instruction::RET(Condition::NotCarry)
        let mut cpu = get_new_cpu();
        cpu.registers.sp = 0x0002;
        cpu.registers.set_flag(Flag::C, false);
        prepare_memory_word(&mut cpu, 0x0002, 0x0101);
        cpu.execute_instruction(Instruction::RET(Condition::NotCarry));
        assert_eq!(cpu.registers.pc, 0x0101);
    }

    #[test]
    fn can_correctly_run_or_instructions() {
        // Instruction::OR(SourceType::Register(Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        cpu.registers.b = 0b0001_0101;
        cpu.execute_instruction(Instruction::OR(SourceType::Register(
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0001_0101);
        assert!(!cpu.registers.get_flag(Flag::H));
        assert!(!cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::N));
        // Instruction::OR(SourceType::Register(Register::B)) Zero
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0000;
        cpu.registers.b = 0b0000_0000;
        cpu.execute_instruction(Instruction::OR(SourceType::Register(
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert!(cpu.registers.get_flag(Flag::Z));
        // Instruction::OR(SourceType::Address(Address::HL))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        cpu.registers.set_hl(0x0001);
        prepare_memory(&mut cpu, 0x0001, 0b0000_0111);
        cpu.execute_instruction(Instruction::OR(SourceType::Address(
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0111);
        // Instruction::OR(SourceType::Value(Value::D8))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        prepare_memory(&mut cpu, 0x0000, 0b0000_0111);
        cpu.execute_instruction(Instruction::OR(SourceType::Value(
            Value::D8,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0111);
    }

    #[test]
    fn can_correctly_run_xor_instructions() {
        // Instruction::XOR(SourceType::Register(Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0001_0100;
        cpu.registers.b = 0b0001_0101;
        cpu.execute_instruction(Instruction::XOR(SourceType::Register(
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0001);
        assert!(!cpu.registers.get_flag(Flag::H));
        assert!(!cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::N));
        // Instruction::XOR(SourceType::Register(Register::B)) Zero
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_1111;
        cpu.registers.b = 0b0000_1111;
        cpu.execute_instruction(Instruction::XOR(SourceType::Register(
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert!(cpu.registers.get_flag(Flag::Z));
        // Instruction::XOR(SourceType::Address(Address::HL))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        cpu.registers.set_hl(0x0001);
        prepare_memory(&mut cpu, 0x0001, 0b0000_0111);
        cpu.execute_instruction(Instruction::XOR(SourceType::Address(
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0011);
        // Instruction::XOR(SourceType::Value(Value::D8))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        prepare_memory(&mut cpu, 0x0000, 0b0000_0111);
        cpu.execute_instruction(Instruction::XOR(SourceType::Value(
            Value::D8,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0011);
    }

    #[test]
    fn can_correctly_run_and_instructions() {
        // Instruction::AND(SourceType::Register(Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        cpu.registers.b = 0b0001_0101;
        cpu.execute_instruction(Instruction::AND(SourceType::Register(
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0100);
        assert!(cpu.registers.get_flag(Flag::H));
        assert!(!cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::N));
        // Instruction::AND(SourceType::Register(Register::B)) Zero
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        cpu.registers.b = 0b0001_0001;
        cpu.execute_instruction(Instruction::AND(SourceType::Register(
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert!(cpu.registers.get_flag(Flag::Z));
        // Instruction::AND(SourceType::Address(Address::HL))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        cpu.registers.set_hl(0x0001);
        prepare_memory(&mut cpu, 0x0001, 0b0000_0111);
        cpu.execute_instruction(Instruction::AND(SourceType::Address(
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0100);
        // Instruction::AND(SourceType::Value(Value::D8))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0100;
        prepare_memory(&mut cpu, 0x0000, 0b0000_0111);
        cpu.execute_instruction(Instruction::AND(SourceType::Value(
            Value::D8,
        )));
        assert_eq!(cpu.registers.a, 0b0000_0100);
    }

    #[test]
    fn can_correctly_run_cpl_instructions() {
        let mut cpu = get_new_cpu();
        // Instruction::CPL
        cpu.registers.a = 0b1010_0010;
        cpu.execute_instruction(Instruction::CPL);
        assert_eq!(cpu.registers.a, 0b0101_1101);
        assert!(cpu.registers.get_flag(Flag::H));
        assert!(cpu.registers.get_flag(Flag::N));
    }

    #[test]
    fn can_correctly_run_ccf_instructions() {
        let mut cpu = get_new_cpu();
        // Instruction::CCF carry false
        cpu.execute_instruction(Instruction::CCF);
        assert!(cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::H));
        assert!(!cpu.registers.get_flag(Flag::N));
        // Instruction::CCF carry true
        let mut cpu = get_new_cpu();
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::CCF);
        assert!(!cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::H));
        assert!(!cpu.registers.get_flag(Flag::N));
    }

    #[test]
    fn can_correctly_run_scf_instructions() {
        let mut cpu = get_new_cpu();
        // Instruction::SCF
        cpu.execute_instruction(Instruction::SCF);
        assert!(cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::H));
        assert!(!cpu.registers.get_flag(Flag::N));
    }

    #[test]
    fn can_correctly_run_daa_instructions() {
        let mut cpu = get_new_cpu();
        // Simulate 0x90 + 0x90 = 0x120 (0x20 + carry)
        // BCD needs to adjust to 0x180 with carry to true
        cpu.registers.a = 0x20;
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::DAA);
        assert_eq!(cpu.registers.a, 0x80);
        assert!(cpu.registers.get_flag(Flag::C));
    }

    #[test]
    fn can_correctly_run_adc_instructions() {
        // Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x01;
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::ADC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x03);
        // Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x01;
        cpu.execute_instruction(Instruction::ADC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x02);
        // Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::B)) carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0xfe;
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::ADC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_flag(Flag::C));
        // Instruction::ADC(OperationType::AddressToRegister(Register::A, Address::HL))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0xfe);
        cpu.registers.set_flag(Flag::C, true);
        cpu.registers.set_hl(0x0000);
        cpu.registers.a = 0x01;
        cpu.execute_instruction(Instruction::ADC(OperationType::AddressToRegister(
            Register::A,
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_flag(Flag::C));
        // Instruction::ADC(OperationType::ValueToRegister(Register::A, Value::D8))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0011);
        cpu.registers.a = 0x0001;
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::ADC(OperationType::ValueToRegister(Register::A, Value::D8)));
        assert_eq!(cpu.registers.a, 0x0013);
    }

    #[test]
    fn can_correctly_run_sbc_instructions() {
        // Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0c;
        cpu.registers.b = 0x01;
        cpu.execute_instruction(Instruction::SBC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x0b);
        assert!(cpu.registers.get_flag(Flag::N));
        // Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::B)) carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0c;
        cpu.registers.b = 0x01;
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::SBC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x0a);
        assert!(cpu.registers.get_flag(Flag::N));
        // Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::B)) half carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0001_0000;
        cpu.registers.b = 0b0000_0001;
        cpu.execute_instruction(Instruction::SBC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_1111);
        assert!(cpu.registers.get_flag(Flag::N));
        assert!(cpu.registers.get_flag(Flag::H));
        // Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::B)) zero
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x02;
        cpu.registers.b = 0x01;
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::SBC(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_flag(Flag::N));
        assert!(cpu.registers.get_flag(Flag::Z));
        // Instruction::SBC(OperationType::AddressToRegister(Register::A, Address::HL))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0c;
        cpu.registers.set_hl(0x0002);
        prepare_memory(&mut cpu, 0x0002, 0x01);
        cpu.execute_instruction(Instruction::SBC(OperationType::AddressToRegister(
            Register::A,
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0x0b);
        assert!(cpu.registers.get_flag(Flag::N));
        // Instruction::SBC(OperationType::ValueToRegister(Register::A, Value::D8))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x03;
        prepare_memory(&mut cpu, 0x0000, 0x0001);
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::SBC(OperationType::ValueToRegister(Register::A, Value::D8)));
        assert_eq!(cpu.registers.a, 0x01);
    }

    #[test]
    fn can_correctly_run_cp_instructions() {
        // Since cp is basically without the results, we only test it once and make sure
        // results are thrown away
        // Instruction::CP(SourceType::Register(Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0c;
        cpu.registers.b = 0x0c;
        cpu.execute_instruction(Instruction::CP(SourceType::Register(Register::B)));
        assert_eq!(cpu.registers.a, 0x0c);
        assert!(cpu.registers.get_flag(Flag::Z));
    }

    #[test]
    fn can_correctly_run_sub_instructions() {
        // Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0c;
        cpu.registers.b = 0x01;
        cpu.execute_instruction(Instruction::SUB(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x0b);
        assert!(cpu.registers.get_flag(Flag::N));
        // Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::B)) zero
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x01;
        cpu.execute_instruction(Instruction::SUB(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_flag(Flag::N));
        assert!(cpu.registers.get_flag(Flag::Z));
        // Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::B)) half carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0001_0000;
        cpu.registers.b = 0b0000_0001;
        cpu.execute_instruction(Instruction::SUB(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0000_1111);
        assert!(cpu.registers.get_flag(Flag::N));
        assert!(cpu.registers.get_flag(Flag::H));
        // Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::B)) carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0000;
        cpu.registers.b = 0b0000_0001;
        cpu.execute_instruction(Instruction::SUB(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b1111_1111);
        assert!(cpu.registers.get_flag(Flag::N));
        assert!(cpu.registers.get_flag(Flag::C));
        // Instruction::SUB(OperationType::AddressToRegister(Register::A, Address::HL))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0c;
        cpu.registers.set_hl(0x0002);
        prepare_memory(&mut cpu, 0x0002, 0x01);
        cpu.execute_instruction(Instruction::SUB(OperationType::AddressToRegister(
            Register::A,
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0x0b);
        assert!(cpu.registers.get_flag(Flag::N));
    }

    #[test]
    fn can_correctly_run_add_instructions() {
        // Instruction::ADD(OperationType::RegisterToRegister Register::HL
        let mut cpu = get_new_cpu();
        cpu.registers.set_bc(0x0100);
        cpu.registers.set_hl(0x0011);
        cpu.execute_instruction(Instruction::ADD(OperationType::RegisterToRegister(
            Register::HL,
            Register::BC,
        )));
        assert_eq!(cpu.registers.get_hl(), 0x0111);
        // Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.b = 0x0c;
        cpu.registers.a = 0x10;
        cpu.execute_instruction(Instruction::ADD(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x1c);
        // Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::B)) zero carry
        let mut cpu = get_new_cpu();
        cpu.registers.b = 0xff;
        cpu.registers.a = 0x01;
        cpu.execute_instruction(Instruction::ADD(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_flag(Flag::Z));
        assert!(cpu.registers.get_flag(Flag::C));
        // Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::B)) half carry
        let mut cpu = get_new_cpu();
        cpu.registers.b = 0b0000_1111;
        cpu.registers.a = 0b0000_0001;
        cpu.execute_instruction(Instruction::ADD(OperationType::RegisterToRegister(
            Register::A,
            Register::B,
        )));
        assert_eq!(cpu.registers.a, 0b0001_0000);
        assert!(cpu.registers.get_flag(Flag::H));
        // Instruction::ADD(OperationType::AddressToRegister(Register::A, Address::HL))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0c);
        cpu.registers.set_hl(0x0000);
        cpu.registers.a = 0x10;
        cpu.execute_instruction(Instruction::ADD(OperationType::AddressToRegister(
            Register::A,
            Address::HL,
        )));
        assert_eq!(cpu.registers.a, 0x1c);
        // Instruction::ADD(OperationType::ValueToRegister(Register::A, Value::D8))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x01;
        prepare_memory(&mut cpu, 0x0000, 0x0c);
        cpu.execute_instruction(Instruction::ADD(OperationType::ValueToRegister(
            Register::A,
            Value::D8,
        )));
        assert_eq!(cpu.registers.a, 0x0d);
        // Instruction::ADD(OperationType::ValueToRegister(Register::SP, Value::R8))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x01);
        cpu.registers.sp = 0x00ff;
        cpu.execute_instruction(Instruction::ADD(OperationType::ValueToRegister(
            Register::SP,
            Value::R8,
        )));
        assert_eq!(cpu.registers.sp, 0x0100);
        assert!(!cpu.registers.get_flag(Flag::N));
        assert!(!cpu.registers.get_flag(Flag::Z));
        // Instruction::ADD(OperationType::ValueToRegister(Register::SP, Value::R8)) carry
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x01);
        cpu.registers.sp = 0xffff;
        cpu.execute_instruction(Instruction::ADD(OperationType::ValueToRegister(
            Register::SP,
            Value::R8,
        )));
        assert_eq!(cpu.registers.sp, 0x0000);
        assert!(cpu.registers.get_flag(Flag::C));
        assert!(!cpu.registers.get_flag(Flag::N));
        assert!(!cpu.registers.get_flag(Flag::Z));
        // Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::B)) half carry
        let mut cpu = get_new_cpu();
        cpu.registers.sp = 0b0000_0000_1111_1111;
        prepare_memory(&mut cpu, 0x0000, 0x01);
        cpu.execute_instruction(Instruction::ADD(OperationType::ValueToRegister(
            Register::SP,
            Value::R8,
        )));
        assert_eq!(cpu.registers.sp, 0b0000_0001_0000_0000);
        assert!(cpu.registers.get_flag(Flag::H));
    }

    #[test]
    fn can_correctly_run_rlca_instructions() {
        // Instruction::RLCA
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0010;
        cpu.execute_instruction(Instruction::RLCA);
        assert_eq!(cpu.registers.a, 0b0000_0100);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        // Instruction::RLCA Carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b1000_0010;
        cpu.execute_instruction(Instruction::RLCA);
        assert_eq!(cpu.registers.a, 0b0000_0101);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn can_correctly_run_jr_instructions() {
        // Instruction::JR(Condition::Always, Address::R8)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0010);
        cpu.execute_instruction(Instruction::JR(Condition::Always, Address::R8));
        assert_eq!(cpu.registers.pc, 0x0011);
        // Instruction::JR(Condition::Carry, Address::R8)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0010);
        cpu.registers.set_flag(Flag::C, true);
        cpu.execute_instruction(Instruction::JR(Condition::Carry, Address::R8));
        assert_eq!(cpu.registers.pc, 0x0011);
        // Instruction::JR(Condition::NotCarry, Address::R8)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0010);
        cpu.execute_instruction(Instruction::JR(Condition::NotCarry, Address::R8));
        assert_eq!(cpu.registers.pc, 0x0011);
        // Instruction::JR(Condition::Zero, Address::R8)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0010);
        cpu.registers.set_flag(Flag::Z, true);
        cpu.execute_instruction(Instruction::JR(Condition::Zero, Address::R8));
        assert_eq!(cpu.registers.pc, 0x0011);
        // Instruction::JR(Condition::Zero, Address::R8)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0x0010);
        cpu.registers.set_flag(Flag::Z, false);
        cpu.execute_instruction(Instruction::JR(Condition::Zero, Address::R8));
        assert_eq!(cpu.registers.pc, 0x0001);
    }

    #[test]
    fn can_correctly_run_rla_instructions() {
        // Instruction::RLA
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0010;
        cpu.execute_instruction(Instruction::RLA);
        assert_eq!(cpu.registers.a, 0b0000_0100);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        // Instruction::RLA Carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b1000_0010;
        cpu.execute_instruction(Instruction::RLA);
        assert_eq!(cpu.registers.a, 0b0000_0100);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn can_correctly_run_rra_instructions() {
        // Instruction::RRA
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0010;
        cpu.execute_instruction(Instruction::RRA);
        assert_eq!(cpu.registers.a, 0b0000_0001);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        // Instruction::RRA Carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0011;
        cpu.execute_instruction(Instruction::RRA);
        assert_eq!(cpu.registers.a, 0b0000_0001);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn can_correctly_run_rrca_instructions() {
        // Instruction::RRCA
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_1000;
        cpu.execute_instruction(Instruction::RRCA);
        assert_eq!(cpu.registers.a, 0b0000_0100);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        // Instruction::RRCA Carry
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0b0000_0011;
        cpu.execute_instruction(Instruction::RRCA);
        assert_eq!(cpu.registers.a, 0b1000_0001);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn can_correctly_run_dec_instructions() {
        // Instruction::DEC(TargetType::Register(Register::B))
        let mut cpu = get_new_cpu();
        cpu.registers.b = 0xfc;
        cpu.execute_instruction(Instruction::DEC(TargetType::Register(Register::B)));
        assert_eq!(cpu.registers.b, 0xfb);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        // Instruction::DEC(TargetType::Register(Register::B)) ZERO
        let mut cpu = get_new_cpu();
        cpu.registers.b = 0x01;
        cpu.execute_instruction(Instruction::DEC(TargetType::Register(Register::B)));
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::Z), true);
        // Instruction::DEC(TargetType::Register(Register::B)) HC
        let mut cpu = get_new_cpu();
        cpu.registers.b = 0x10;
        cpu.execute_instruction(Instruction::DEC(TargetType::Register(Register::B)));
        assert_eq!(cpu.registers.b, 0x0f);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        // Instruction::DEC(TargetType::Address(Address::HL))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0001, 0xfe);
        cpu.registers.set_hl(0x0001);
        cpu.execute_instruction(Instruction::DEC(TargetType::Address(Address::HL)));
        assert_eq!(cpu.memory.borrow().get(0x0001), 0xfd);
    }

    #[test]
    fn can_correctly_run_ld_instructions() {
        // Instruction::LD(OperationType::ValueToRegister)
        let mut cpu = get_new_cpu();
        prepare_memory_word(&mut cpu, 0x0000, 0x0101);
        cpu.execute_instruction(Instruction::LD(OperationType::ValueToRegister(
            Register::BC,
            Value::D16,
        )));
        assert_eq!(cpu.registers.get_bc(), 0x0101);
        // Instruction::LD(OperationType::RegisterToAddress)
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0xff;
        cpu.execute_instruction(Instruction::LD(OperationType::RegisterToAddress(
            Address::BC,
            Register::A,
        )));
        assert_eq!(cpu.memory.borrow().get(cpu.registers.get_bc()), 0xff);
        // Instruction::LD(OperationType::AddressToRegister)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0001, 0xfe);
        cpu.registers.set_de(0x0001);
        cpu.execute_instruction(Instruction::LD(OperationType::AddressToRegister(
            Register::A,
            Address::DE,
        )));
        assert_eq!(cpu.registers.a, 0xfe);
        // Instruction::LD(OperationType::ValueToAddress)
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0000, 0xfe);
        cpu.registers.set_hl(0x0001);
        cpu.execute_instruction(Instruction::LD(OperationType::ValueToAddress(
            Address::HL,
            Value::D8,
        )));
        assert_eq!(cpu.memory.borrow().get(0x0001), 0xfe);
        // Instruction::LD(OperationType::RegisterToRegister)
        let mut cpu = get_new_cpu();
        cpu.registers.e = 0xfc;
        cpu.registers.a = 0x01;
        cpu.execute_instruction(Instruction::LD(OperationType::RegisterToRegister(
            Register::A,
            Register::E,
        )));
        assert_eq!(cpu.registers.a, 0xfc);
        // Instruction::LD(OperationType::RegisterToAddress(Address::A16, Register::A))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x1c;
        prepare_memory_word(&mut cpu, 0x0000, 0x001c);
        cpu.execute_instruction(Instruction::LD(OperationType::RegisterToAddress(Address::A16, Register::A)));
        assert_eq!(cpu.memory.borrow().get(0x001c), 0x1c);
    }

    #[test]
    fn get_correctly_run_inc_instructions() {
        // Instruction::INC(TargetType::Register(Register::A))
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x01;
        cpu.execute_instruction(Instruction::INC(TargetType::Register(Register::A)));
        assert_eq!(cpu.registers.a, 0x02);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        // Instruction::INC(TargetType::Register(Register::A)) ZERO
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0xff;
        cpu.execute_instruction(Instruction::INC(TargetType::Register(Register::A)));
        assert_eq!(cpu.registers.a, 0x00);
        assert!(cpu.registers.get_flag(Flag::Z));
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        // Instruction::INC(TargetType::Register(Register::A)) HC
        let mut cpu = get_new_cpu();
        cpu.registers.a = 0x0f;
        cpu.execute_instruction(Instruction::INC(TargetType::Register(Register::A)));
        assert_eq!(cpu.registers.a, 0x10);
        assert!(cpu.registers.get_flag(Flag::H));
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        // Instruction::INC(TargetType::Register(Register::DE))
        let mut cpu = get_new_cpu();
        cpu.registers.set_de(0xfffc);
        cpu.execute_instruction(Instruction::INC(TargetType::Register(Register::DE)));
        assert_eq!(cpu.registers.get_de(), 0xfffd);
        // Instruction::INC(TargetType::Address(Address::HL))
        let mut cpu = get_new_cpu();
        prepare_memory(&mut cpu, 0x0001, 0xfe);
        cpu.registers.set_hl(0x0001);
        cpu.execute_instruction(Instruction::INC(TargetType::Address(Address::HL)));
        assert_eq!(cpu.memory.borrow().get(0x0001), 0xff);
    }
}
