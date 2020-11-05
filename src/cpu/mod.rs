pub mod instruction;
pub mod opcodes;
pub mod registers;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

pub use self::instruction::{
    Address, Condition, Instruction, OperationType, Register, SourceType, TargetType, Value,
};
use self::registers::{Flag, Registers};
use crate::memory::Memory;

// Nintendo documents describe the CPU & instructions speed in machine cycles while this document describes them in
// clock cycles. Here is the translation:
//   1 machine cycle = 4 clock cycles
//                   GB CPU Speed    NOP Instruction
// Machine Cycles    1.05MHz         1 cycle
// Clock Cycles      4.19MHz         4 cycles
//
//  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
const OP_CYCLES: [u32; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // a
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // b
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // c
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // d
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // e
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // f
];

//  0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
const CB_CYCLES: [u32; 256] = [
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 0
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 1
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 2
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 3
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 4
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 5
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 6
    2, 2, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 3, 2, // 7
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 8
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // 9
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // a
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // b
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // c
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // d
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // e
    2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, 4, 2, // f
];

pub struct CPU {
    pub registers: Registers,
    memory: Rc<RefCell<dyn Memory>>,
    pub last_instruction: Instruction,
    is_halted: bool,
    interrupts_enabled: bool,
}

enum DataType {
    U8(u8),
    U16(u16),
}

impl CPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> CPU {
        CPU {
            registers: Registers::new(),
            memory,
            last_instruction: Instruction::NOP,
            is_halted: true,
            interrupts_enabled: true,
        }
    }
    fn get_next(&mut self) -> u8 {
        let opcode = self.memory.borrow().get(self.registers.pc);
        self.registers.pc += 1;
        opcode
    }
    fn get_next_word(&mut self) -> u16 {
        let opcode = self.memory.borrow().get_word(self.registers.pc);
        self.registers.pc += 2;
        opcode
    }
    // Test bit b in register r.
    // b = 0 - 7, r = A,B,C,D,E,H,L,(HL)
    //
    // Flags affected:
    // Z - Set if bit b of register r is 0.
    // N - Reset.
    // H - Set.
    // C - Not affected
    fn alu_bit(&mut self, value: u8, position: u8) {
        let r = value & (1 << position) == 0x00;
        self.registers.set_flag(Flag::H, true);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, r);
    }
    // Add n to current address and jump to it.
    // n = one byte signed immediate value
    fn alu_jr(&mut self, n: u8) {
        let n = n as i8;
        self.registers.pc = ((u32::from(self.registers.pc) as i32) + i32::from(n)) as u16;
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
                panic!("Invalid assignment to register");
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
    fn value_to_register(&mut self, register: Register, value: Value) {
        match value {
            Value::D16 => {
                let d16 = self.get_next_word();
                trace!("LD {}, ${:x}", register, d16);

                // Set the register value
                self.set_register_16(register, d16);
            }
            Value::D8 => {
                let d8 = self.get_next();
                trace!("LD {}, ${:x}", register, d8);
                // Set the register value
                self.set_register(register, d8);
            }
            _ => {
                panic!("Not implemented: {:?} <- {:?}", register, value);
            }
        }
    }
    fn value_to_address(&mut self, address: Address, value: Value) {
        match value {
            Value::D8 => {
                let d8 = self.get_next();
                trace!("LD ({}), ${:x}", address, d8);
                // Set the register value
                self.set_address_value(address, d8);
            }
            _ => {
                panic!("Not implemented: ({:?}) <- {:?}", address, value);
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
    fn get_address(&mut self, address: Address) -> u8 {
        match address {
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
    fn set_address_value(&mut self, address: Address, value: u8) {
        match address {
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
    fn xor(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    pub fn tick(&mut self) -> u32 {
        let mut instruction_byte = self.get_next();

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.get_next();
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed);

        self.last_instruction = instruction;

        debug!(
            "HEX: {:x} Decoded: {:?} Prefixed: {}",
            instruction_byte, instruction, prefixed
        );

        match instruction {
            Instruction::NAI => {
                panic!("Not suppose to run the NAI instruction");
            }
            Instruction::NOP => {
                // Not doing anything for NOP
            }
            Instruction::LD(operation_type) => match operation_type {
                // Finished ✔
                OperationType::ValueToRegister(register, value) => {
                    self.value_to_register(register, value)
                }
                OperationType::ValueToAddress(address, value) => {
                    self.value_to_address(address, value);
                }
                OperationType::RegisterToAddress(address, register) => {
                    self.register_to_address(address, register);
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
                OperationType::AddressToRegister(register, address) => {
                    self.address_to_register(register, address);
                }
                _ => {
                    panic!("Not implemented: {:?}", instruction);
                }
            },
            Instruction::LDH(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::ADD(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::AND(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::ADC(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SBC(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::XOR(source_type) => match source_type {
                SourceType::Register(register) => {
                    trace!("XOR {}", register);
                    match register {
                        Register::A => {
                            self.xor(self.registers.a);
                        }
                        _ => {
                            panic!("Not implemented: {:?}", instruction);
                        }
                    }
                }
                _ => {
                    panic!("Not implemented: {:?}", instruction);
                }
            },
            Instruction::OR(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::CP(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SUB(operation_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::INC(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::DEC(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::JR(condition, _) => {
                // Finished ✔
                let can_jump = match condition {
                    Condition::NotZero => !self.registers.get_flag(Flag::Z),
                    Condition::Zero => self.registers.get_flag(Flag::Z),
                    Condition::Carry => self.registers.get_flag(Flag::C),
                    Condition::NotCarry => !self.registers.get_flag(Flag::C),
                    Condition::Always => true,
                };
                let n = self.get_next();
                if can_jump {
                    self.alu_jr(n);
                }
            }
            Instruction::JP(condition, address) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::CALL(condition, address) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RST(condition) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::PUSH(register) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::POP(register) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RET(condition) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::EI => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::DI => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::CPL => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::CCF => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SCF => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RLA => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::DAA => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RRA => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::HALT => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RLCA => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RRCA => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RETI => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::STOP => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RL(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RR(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RLC(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::RRC(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SLA(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SRA(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SRL(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SWAP(target_type) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::BIT(target_type, location) => {
                // Finished ✔
                match target_type {
                    TargetType::Register(register) => {
                        trace!("BIT {}, {}", location as u8, register);
                        let register_value = self.get_register(register);
                        match register_value {
                            DataType::U8(value) => {
                                self.alu_bit(value, location as u8);
                            }
                            DataType::U16(_) => {
                                panic!("Invalid datatype u16");
                            }
                        }
                    }
                    TargetType::Address(address) => match address {
                        Address::HL => {
                            let address_value = self.memory.borrow().get(self.registers.get_hl());
                            self.alu_bit(address_value, location as u8);
                        }
                        _ => {
                            panic!("Invalid address value");
                        }
                    },
                }
            }
            Instruction::RES(target_type, location) => {
                panic!("Not implemented: {:?}", instruction);
            }
            Instruction::SET(target_type, location) => {
                panic!("Not implemented: {:?}", instruction);
            }
            _ => {
                panic!("Unknown instruction: {:?}", instruction);
            }
        }

        self.print_registers();

        let ecycle = match instruction_byte {
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

        if prefixed {
            CB_CYCLES[instruction_byte as usize]
        } else {
            OP_CYCLES[instruction_byte as usize] + ecycle
        }
    }
    fn print_registers(&self) {
        debug!("{:?}", self.registers);
    }
}

pub struct ClockedCPU {
    pub cpu: CPU,
    pub frequency: u32,
    pub last_ran: u128,
    pub cycle_time: u128,
    pub wait_time: u128,
    last_cycle: u128,
    pub speed: f32,
    pub cycle_duration: u128,
}

impl ClockedCPU {
    pub fn new(frequency: u32, speed: f32, memory: Rc<RefCell<dyn Memory>>) -> ClockedCPU {
        ClockedCPU {
            frequency,
            cpu: CPU::new(memory),
            cycle_time: ((1 as f64 / frequency as f64) * (1_000_000_000 as f64)) as u128, // Cycletime in nano seconds
            last_ran: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            wait_time: 0,
            speed,
            last_cycle: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            cycle_duration: 0,
        }
    }

    pub fn update_freq(&mut self, frequency: u32) {}

    pub fn tick(&mut self) -> u32 {
        if self.wait_time > 0 {
            let delta = SystemTime::now() // This will be the nano seconds taken for each cycle
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                - self.last_ran;
            match self.wait_time.checked_sub(delta) {
                Some(w) => self.wait_time = w,
                None => self.wait_time = 0,
            }
            self.last_ran = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            return 0;
        } else {
            let cycles = self.cpu.tick();
            let delta = SystemTime::now() // This will be the nano seconds taken for each cycle
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                - self.last_ran;
            let expected_cycle_time = cycles as u128 * self.cycle_time;
            if delta < expected_cycle_time {
                self.wait_time = ((expected_cycle_time - delta) as f64 / self.speed as f64) as u128;
            }
            self.last_ran = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            self.cycle_duration = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                - self.last_cycle;
            self.last_cycle = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            cycles
        }
    }
}
