pub mod instruction;
pub mod opcodes;
pub mod registers;

use std::cell::RefCell;
use std::rc::Rc;

pub use self::instruction::{Condition, Instruction, OperationType, Register, SourceType, Value};
use self::registers::{Flag, Registers};
use crate::memory::Memory;

pub struct CPU {
    pub registers: Registers,
    memory: Rc<RefCell<dyn Memory>>,
    is_halted: bool,
    interrupts_enabled: bool,
}

impl CPU {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> CPU {
        CPU {
            registers: Registers::new(),
            memory,
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
    fn set_reg_word(&mut self, register: Register, value: u16) {
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
    fn xor(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        self.registers.set_flag(Flag::C, false);
        self.registers.set_flag(Flag::H, false);
        self.registers.set_flag(Flag::N, false);
        self.registers.set_flag(Flag::Z, result == 0x00);
        self.registers.a = result;
    }
    pub fn next(&mut self) -> u32 {
        let mut instruction_byte = self.get_next();

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.get_next();
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed);

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
            Instruction::LD(operation_type) => {
                // panic!("Not implemented: {:?}", instruction);
                match operation_type {
                    OperationType::ValueToRegister(register, value) => match value {
                        Value::D16 => {
                            let d16 = self.get_next_word();
                            trace!("LD {}, ${:x}", register, d16);

                            // Set the register value
                            self.set_reg_word(register, d16);
                        }
                        _ => {
                            panic!("Not implemented: {:?}", instruction);
                        }
                    },
                    OperationType::RegisterToAddress(address, register) => {}
                    _ => {
                        panic!("Not implemented: {:?}", instruction);
                    }
                }
            }
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
            Instruction::JR(condition, address) => {
                panic!("Not implemented: {:?}", instruction);
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
            Instruction::PREFIX => {
                let prefix_instruction = Instruction::NAI;
                match prefix_instruction {
                    // Prefixed
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
                        panic!("Not implemented: {:?}", instruction);
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
            }
            _ => {
                panic!("Unknown instruction: {:?}", instruction);
            }
        }

        self.print_registers();
        0
    }
    fn print_registers(&self) {
        debug!("{:?}", self.registers);
    }
}
