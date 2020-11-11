use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

use super::cycles::{CB_CYCLES, OP_CYCLES};
use super::instruction::{Address, Instruction, OperationType, Register, TargetType, Value};
use super::registers::{Flag, Registers};
use piston_window::math::add;

pub struct Core {
    pub memory: Rc<RefCell<dyn Memory>>,
    pub registers: Registers,
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
            Instruction::NOP => {} // Not doing anything in NOP
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
                                panic!("Invalid datatype u16");
                            }
                        }
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
            Instruction::RLCA => {
                self.registers.a = self.alu_rlc(self.registers.a);
                self.registers.set_flag(Flag::Z, false);
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
        pub memory: [u8; 0xff],
    }

    impl TestMemory {
        pub fn new() -> TestMemory {
            Self { memory: [0; 0xff] }
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
