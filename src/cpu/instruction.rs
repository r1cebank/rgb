use crate::cpu::opcodes;
use strum_macros::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
  INC(IncDecOperationType),
  DEC(IncDecOperationType),
  ADD(OperationType),
  SUB(OperationType),
  ADC(OperationType),
  SBC(OperationType),
  JP(JumpCondition),
  JR(JumpCondition, JumpSource),
  LD(LoadType),

  NAI,
  CPL,
  CCF,
  SCF,
  RLA,
  DAA,
  RRA,
  NOP,
  HALT,
  RLCA,
  RRCA,
  STOP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum IncDecOperationType {
  ToRegister(IncDecTarget),
  ToAddress(IncDecTarget),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum OperationType {
  ToRegister(ArithmeticTarget, ArithmeticSource),
  FromAddress(ArithmeticTarget, ArithmeticSource),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum LoadType {
  ToRegister(LoadTarget, LoadSource),
  FromAddress(LoadTarget, AddressSource),
  ToAddress(AddressTarget, LoadSource),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum IncDecTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  BC,
  DE,
  HL,
  SP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum LoadSource {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  D8,
  D16,
  BC,
  DE,
  SP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum AddressSource {
  BC,
  DE,
  HL,
  HLP,
  HLM,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum LoadTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  HL,
  BC,
  DE,
  SP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum AddressTarget {
  BC,
  DE,
  A16,
  HL,
  HLP,
  HLM,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum JumpSource {
  R8,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum ArithmeticSource {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  D8,
  DE,
  BC,
  SP,
  HL,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum ArithmeticTarget {
  A,
  HL,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum JumpCondition {
  NotZero,
  NotCarry,
  Zero,
  Carry,
  Always,
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Instruction {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  pub fn from_byte_prefixed(byte: u8) -> Instruction {
    opcodes::PREFIX_INSTRUCTION_MAP[(byte >> 4 & 0xf) as usize][(byte & 0xf) as usize]
  }

  pub fn from_byte_not_prefixed(byte: u8) -> Instruction {
    opcodes::INSTRUCTION_MAP[(byte >> 4 & 0xf) as usize][(byte & 0xf) as usize]
  }
}

#[cfg(test)]
mod tests {
  extern crate serde;
  extern crate serde_json;

  use super::*;
  use serde::Deserialize;
  use std::collections::HashMap;
  use std::fs::File;
  use std::io::Read;

  #[derive(Deserialize, Clone, Debug)]
  struct Operand {
    name: String,
    immediate: bool,
    #[serde(default)]
    decrement: bool,
    #[serde(default)]
    increment: bool,
  }

  #[derive(Deserialize, Debug)]
  struct Opcode {
    mnemonic: String,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Vec<Operand>,
    immediate: bool,
  }

  #[derive(Deserialize, Debug)]
  struct Opcodes {
    unprefixed: HashMap<String, Opcode>,
    cbprefixed: HashMap<String, Opcode>,
  }

  fn assert_operand(truth: Operand, operand: String) {
    if truth.decrement {
      assert_eq!(format!("{}M", truth.name.to_uppercase()), operand);
    } else if truth.increment {
      assert_eq!(format!("{}P", truth.name.to_uppercase()), operand);
    } else {
      assert_eq!(format!("{}", truth.name.to_uppercase()), operand);
    }
  }

  #[test]
  /// Use the opcode json from https://gbdev.io/gb-opcodes//optables/classic
  /// to ensure the opcode is decoded correctly
  fn can_correctly_decode_opcode() {
    let mut file = File::open("res/opcodes.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let opcodes: Opcodes = serde_json::from_str(&contents).unwrap();

    for (opcode, reference_opcode) in opcodes.unprefixed {
      let opcode_u8 = u8::from_str_radix(opcode.as_str().trim_start_matches("0x"), 16).unwrap();
      let instruction = Instruction::from_byte_not_prefixed(opcode_u8);
      if instruction != Instruction::NAI {
        print!("Checking: {:?}", instruction);
        match instruction {
          Instruction::NOP => {
            assert_eq!(reference_opcode.mnemonic, "NOP");
          }
          Instruction::INC(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "INC");
            match operation_type {
              IncDecOperationType::ToRegister(target) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              }
              IncDecOperationType::ToAddress(target) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              }
            }
          }
          Instruction::ADD(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "ADD");
            match operation_type {
              OperationType::ToRegister(target, source) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
                assert_operand(reference_opcode.operands[1].clone(), source.to_string());
              }
              OperationType::FromAddress(target, source) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
                assert_operand(reference_opcode.operands[1].clone(), source.to_string());
              }
            }
          }
          Instruction::ADC(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "ADC");
            match operation_type {
              OperationType::ToRegister(target, source) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
                assert_operand(reference_opcode.operands[1].clone(), source.to_string());
              }
              OperationType::FromAddress(target, source) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
                assert_operand(reference_opcode.operands[1].clone(), source.to_string());
              }
            }
          }
          Instruction::SUB(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "SUB");
            match operation_type {
              OperationType::ToRegister(target, source) => {
                assert_operand(
                  Operand {
                    increment: false,
                    decrement: false,
                    immediate: true,
                    name: String::from("A"),
                  },
                  target.to_string(),
                );
                assert_operand(reference_opcode.operands[0].clone(), source.to_string());
              }
              OperationType::FromAddress(target, source) => {
                assert_operand(
                  Operand {
                    increment: false,
                    decrement: false,
                    immediate: true,
                    name: String::from("A"),
                  },
                  target.to_string(),
                );
                assert_operand(reference_opcode.operands[0].clone(), source.to_string());
              }
            }
          }
          Instruction::SBC(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "SBC");
            match operation_type {
              OperationType::ToRegister(target, source) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
                assert_operand(reference_opcode.operands[1].clone(), source.to_string());
              }
              OperationType::FromAddress(target, source) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
                assert_operand(reference_opcode.operands[1].clone(), source.to_string());
              }
            }
          }
          Instruction::DEC(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "DEC");
            match operation_type {
              IncDecOperationType::ToRegister(target) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              }
              IncDecOperationType::ToAddress(target) => {
                assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              }
            }
          }
          Instruction::LD(load_type) => match load_type {
            LoadType::ToRegister(target, source) => {
              assert_eq!(reference_opcode.mnemonic, "LD");
              assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              assert_operand(reference_opcode.operands[1].clone(), source.to_string());
            }
            LoadType::ToAddress(target, source) => {
              assert_eq!(reference_opcode.mnemonic, "LD");
              assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              assert_operand(reference_opcode.operands[1].clone(), source.to_string());
            }
            LoadType::FromAddress(target, source) => {
              assert_eq!(reference_opcode.mnemonic, "LD");
              assert_operand(reference_opcode.operands[0].clone(), target.to_string());
              assert_operand(reference_opcode.operands[1].clone(), source.to_string());
            }
          },
          Instruction::JR(condition, source) => match condition {
            JumpCondition::Always => {
              assert_eq!(reference_opcode.mnemonic, "JR");
              assert_eq!(
                reference_opcode.operands[0].name.to_uppercase(),
                source.to_string()
              );
            }
            JumpCondition::NotZero => {
              assert_eq!(reference_opcode.mnemonic, "JR");
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "NZ");
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                source.to_string()
              );
            }
            JumpCondition::Carry => {
              assert_eq!(reference_opcode.mnemonic, "JR");
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "C");
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                source.to_string()
              );
            }
            _ => {}
          },
          Instruction::RLCA => {
            assert_eq!(reference_opcode.mnemonic, "RLCA");
          }
          Instruction::RRCA => {
            assert_eq!(reference_opcode.mnemonic, "RRCA");
          }
          Instruction::RLA => {
            assert_eq!(reference_opcode.mnemonic, "RLA");
          }
          Instruction::RRA => {
            assert_eq!(reference_opcode.mnemonic, "RRA");
          }
          Instruction::DAA => {
            assert_eq!(reference_opcode.mnemonic, "DAA");
          }
          Instruction::SCF => {
            assert_eq!(reference_opcode.mnemonic, "SCF");
          }
          Instruction::CPL => {
            assert_eq!(reference_opcode.mnemonic, "CPL");
          }
          Instruction::CCF => {
            assert_eq!(reference_opcode.mnemonic, "CCF");
          }
          Instruction::STOP => {
            assert_eq!(reference_opcode.mnemonic, "STOP");
          }
          Instruction::HALT => {
            assert_eq!(reference_opcode.mnemonic, "HALT");
          }
          Instruction::NAI => {
            assert_eq!(reference_opcode.mnemonic.contains("ILLEGAL"), true);
          }
          _ => {
            println!("{:?} not tested", instruction);
          }
        }
        println!(" ✔");
      }
    }
  }
}
