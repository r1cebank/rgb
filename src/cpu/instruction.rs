use crate::cpu::opcodes;
use strum_macros::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
  INC(IncDecTarget),
  DEC(IncDecTarget),
  ADD(ArithmeticTarget),
  ADC(ArithmeticTarget),
  ADDHL(HLTarget),
  JP(JumpCondition),

  NAI,
  NOP,
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
  HLI,
  BC,
  DE,
  HL,
  SP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum ArithmeticTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  D8,
  HLI,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum HLTarget {
  BC,
  DE,
  HL,
  SP,
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

  #[derive(Deserialize, Debug)]
  struct OpcodeFlag {
    Z: String,
    N: String,
    H: String,
    C: String,
  }

  #[derive(Deserialize, Debug)]
  struct Operand {
    name: String,
    immediate: bool,
  }

  #[derive(Deserialize, Debug)]
  struct Opcode {
    mnemonic: String,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Vec<Operand>,
    immediate: bool,
    flags: OpcodeFlag,
  }

  #[derive(Deserialize, Debug)]
  struct Opcodes {
    unprefixed: HashMap<String, Opcode>,
    cbprefixed: HashMap<String, Opcode>,
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
        println!("Checking: {:?}", instruction);
        match instruction {
          Instruction::NOP => {
            assert_eq!(reference_opcode.mnemonic, "NOP");
          }
          Instruction::INC(operand) => {
            assert_eq!(reference_opcode.mnemonic, "INC");
            if !reference_opcode.operands[0].immediate {
              assert_eq!(String::from("HLI"), operand.to_string());
            } else {
              assert_eq!(
                reference_opcode.operands[0].name.to_uppercase(),
                operand.to_string()
              );
            }
          }
          Instruction::ADD(operand) => {
            assert_eq!(reference_opcode.mnemonic, "ADD");
            assert_eq!(reference_opcode.operands[0].name, "A");
            if !reference_opcode.operands[1].immediate {
              assert_eq!(String::from("HLI"), operand.to_string());
            } else {
              assert_eq!(reference_opcode.operands[1].name, operand.to_string());
            }
          }
          Instruction::ADDHL(operand) => {
            assert_eq!(reference_opcode.mnemonic, "ADD");
            assert_eq!(reference_opcode.operands[0].name, "HL");
            assert_eq!(reference_opcode.operands[1].name, operand.to_string());
          }
          Instruction::ADC(operand) => {
            assert_eq!(reference_opcode.mnemonic, "ADC");
            assert_eq!(reference_opcode.operands[0].name, "A");
            if !reference_opcode.operands[1].immediate {
              assert_eq!(String::from("HLI"), operand.to_string());
            } else {
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                operand.to_string()
              );
            }
          }
          Instruction::DEC(operand) => {
            assert_eq!(reference_opcode.mnemonic, "DEC");
            if !reference_opcode.operands[0].immediate {
              assert_eq!(String::from("HLI"), operand.to_string());
            } else {
              assert_eq!(
                reference_opcode.operands[0].name.to_uppercase(),
                operand.to_string()
              );
            }
          }
          Instruction::NAI => {
            assert_eq!(reference_opcode.mnemonic.contains("ILLEGAL"), true);
          }
          _ => {
            println!("{:?} not tested", instruction);
          }
        }
      }
    }
  }
}
