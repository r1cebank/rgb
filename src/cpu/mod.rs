pub mod instruction;
pub mod opcodes;
pub mod registers;

use std::cell::RefCell;
use std::rc::Rc;

pub use self::instruction::{Condition, Instruction, OperationType, Register};
use self::registers::Registers;
use crate::memory::Memory;

pub struct CPU {
    pub registers: Registers,
    memory: Memory,
    pc: u16,
    sp: u16,
    is_halted: bool,
    interrupts_enabled: bool,
}

impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>) -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(boot_rom),
            pc: 0x0,
            sp: 0x0,
            is_halted: true,
            interrupts_enabled: true,
        }
    }
    pub fn disassemble_boot(&self) {}
    pub fn execute(&mut self, dry_run: bool) -> u32 {
        let instruction = Instruction::HALT;
        if dry_run {
            println!("{:?}", instruction);
            0
        } else {
            match instruction {
                Instruction::NAI => {
                    panic!("Not suppose to run the NAI instruction");
                }
                Instruction::NOP => {
                    // Not doing anything for NOP
                }
                Instruction::LD(operation_type) => {
                    panic!("Not implemented: {:?}", instruction);
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
                Instruction::XOR(operation_type) => {
                    panic!("Not implemented: {:?}", instruction);
                }
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
            0
        }
    }
}
