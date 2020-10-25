use crate::cpu::opcodes;
use strum_macros::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
  INC(IncDecOperationType),
  DEC(IncDecOperationType),
  OR(ArithmeticOperationType),
  CP(ArithmeticOperationType),
  ADD(ArithmeticOperationType),
  AND(ArithmeticOperationType),
  SUB(ArithmeticOperationType),
  ADC(ArithmeticOperationType),
  SBC(ArithmeticOperationType),
  XOR(ArithmeticOperationType),
  JR(Condition, ConditionSource),
  JP(Condition, ConditionSource),
  RST(AddressLocation),
  CALL(Condition, ConditionSource),
  PUSH(RegisterSource),
  POP(RegisterTarget),
  RET(Condition),
  LD(LoadType),
  LDH(LoadType),
  EI,
  DI,
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
  RETI,
  STOP,
  PREFIX,

  // Prefixed
  RL(BitArthOperationType),
  RR(BitArthOperationType),
  RLC(BitArthOperationType),
  RRC(BitArthOperationType),
  SLA(BitArthOperationType),
  SRA(BitArthOperationType),
  SRL(BitArthOperationType),
  SWAP(BitArthOperationType),
  BIT(BitTestType),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum IncDecOperationType {
  ToRegister(IncDecTarget),
  ToAddress(IncDecTarget),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum BitTestType {
  FromRegister(BitTestLocation, BitTestSource),
  FromAddress(BitTestLocation, BitTestSource),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum BitTestSource {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  HL,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum BitTestLocation {
  B0,
  B1,
  B2,
  B3,
  B4,
  B5,
  B6,
  B7,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum BitArthOperationType {
  ToRegister(BitOperationTarget),
  ToAddress(BitOperationTarget),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum BitOperationTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  HL,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum AddressLocation {
  X00H,
  X08H,
  X10H,
  X18H,
  X20H,
  X28H,
  X30H,
  X38H,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum ArithmeticOperationType {
  ToRegister(ArithmeticTarget, ArithmeticSource),
  FromAddress(ArithmeticTarget, ArithmeticSource),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum LoadType {
  ToRegister(RegisterTarget, LoadSource),
  ToOffsetAddress(AddressTarget, LoadSource),
  FromAddress(RegisterTarget, AddressSource),
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
  HL,
  SP,
  SPP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum RegisterSource {
  AF,
  HL,
  BC,
  DE,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum AddressSource {
  C,
  BC,
  DE,
  HL,
  A8,
  A16,
  HLP,
  HLM,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum RegisterTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  AF,
  HL,
  BC,
  DE,
  SP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum AddressTarget {
  C,
  BC,
  DE,
  A16,
  A8,
  HL,
  HLP,
  HLM,
}
#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum ConditionSource {
  R8,
  HL,
  A16,
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
  R8,
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
  SP,
  HL,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum Condition {
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

  #[derive(Deserialize, Clone, Debug)]
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

  fn assert_operand(truth: Operand, operand: String, custom_msg: String) {
    if truth.decrement {
      assert_eq!(
        format!("{}M", truth.name.to_uppercase()),
        operand,
        "{}",
        custom_msg
      );
    } else if truth.increment {
      assert_eq!(
        format!("{}P", truth.name.to_uppercase()),
        operand,
        "{}",
        custom_msg
      );
    } else {
      assert_eq!(
        format!("{}", truth.name.to_uppercase()),
        operand,
        "{}",
        custom_msg
      );
    }
  }

  fn assert_condition(
    reference_opcode: Opcode,
    condition: Condition,
    source: ConditionSource,
    custom_msg: String,
  ) {
    match condition {
      Condition::Always => {
        assert_eq!(
          reference_opcode.operands[0].name.to_uppercase(),
          source.to_string(),
          "{}",
          custom_msg
        );
      }
      Condition::NotZero => {
        assert_eq!(
          reference_opcode.operands[0].name.to_uppercase(),
          "NZ",
          "{}",
          custom_msg
        );
        assert_eq!(
          reference_opcode.operands[1].name.to_uppercase(),
          source.to_string(),
          "{}",
          custom_msg
        );
      }
      Condition::Carry => {
        assert_eq!(
          reference_opcode.operands[0].name.to_uppercase(),
          "C",
          "{}",
          custom_msg
        );
        assert_eq!(
          reference_opcode.operands[1].name.to_uppercase(),
          source.to_string(),
          "{}",
          custom_msg
        );
      }
      Condition::NotCarry => {
        assert_eq!(
          reference_opcode.operands[0].name.to_uppercase(),
          "NC",
          "{}",
          custom_msg
        );
        assert_eq!(
          reference_opcode.operands[1].name.to_uppercase(),
          source.to_string(),
          "{}",
          custom_msg
        );
      }
      Condition::Zero => {
        assert_eq!(
          reference_opcode.operands[0].name.to_uppercase(),
          "Z",
          "{}",
          custom_msg
        );
        assert_eq!(
          reference_opcode.operands[1].name.to_uppercase(),
          source.to_string(),
          "{}",
          custom_msg
        );
      }
    }
  }

  fn assert_arithmetric(
    reference_opcode: Opcode,
    operation_type: ArithmeticOperationType,
    fixed_target: bool,
    instruction: Instruction,
  ) {
    if fixed_target {
      match operation_type {
        ArithmeticOperationType::ToRegister(target, source) => {
          assert_operand(
            Operand {
              increment: false,
              decrement: false,
              immediate: true,
              name: String::from("A"),
            },
            target.to_string(),
            format!("{:?} failed assert", instruction),
          );
          assert_operand(
            reference_opcode.operands[0].clone(),
            source.to_string(),
            format!("{:?} failed assert", instruction),
          );
        }
        ArithmeticOperationType::FromAddress(target, source) => {
          assert_operand(
            Operand {
              increment: false,
              decrement: false,
              immediate: true,
              name: String::from("A"),
            },
            target.to_string(),
            format!("{:?} failed assert", instruction),
          );
          assert_operand(
            reference_opcode.operands[0].clone(),
            source.to_string(),
            format!("{:?} failed assert", instruction),
          );
        }
      }
    } else {
      match operation_type {
        ArithmeticOperationType::ToRegister(target, source) => {
          assert_operand(
            reference_opcode.operands[0].clone(),
            target.to_string(),
            format!("{:?} failed assert", instruction),
          );
          assert_operand(
            reference_opcode.operands[1].clone(),
            source.to_string(),
            format!("{:?} failed assert", instruction),
          );
        }
        ArithmeticOperationType::FromAddress(target, source) => {
          assert_operand(
            reference_opcode.operands[0].clone(),
            target.to_string(),
            format!("{:?} failed assert", instruction),
          );
          assert_operand(
            reference_opcode.operands[1].clone(),
            source.to_string(),
            format!("{:?} failed assert", instruction),
          );
        }
      }
    }
  }

  fn assert_rotate(
    reference_opcode: Opcode,
    operation_type: BitArthOperationType,
    instruction: Instruction,
  ) {
    match operation_type {
      BitArthOperationType::ToAddress(target) => {
        assert_eq!(
          reference_opcode.operands[0].immediate, false,
          "{:?}",
          instruction
        );
        assert_operand(
          reference_opcode.operands[0].clone(),
          target.to_string(),
          format!("{:?} failed assert", instruction),
        );
      }
      BitArthOperationType::ToRegister(target) => {
        assert_operand(
          reference_opcode.operands[0].clone(),
          target.to_string(),
          format!("{:?} failed assert", instruction),
        );
      }
    }
  }
  fn assert_bit_test(
    reference_opcode: Opcode,
    operation_type: BitTestType,
    instruction: Instruction,
  ) {
    match operation_type {
      BitTestType::FromAddress(location, target) => {
        assert_eq!(
          reference_opcode.operands[1].immediate, false,
          "{:?}",
          instruction
        );
        assert_eq!(
          format!("B{}", reference_opcode.operands[0].name),
          location.to_string(),
          "{:?}",
          instruction
        );
        assert_operand(
          reference_opcode.operands[1].clone(),
          target.to_string(),
          format!("{:?} failed assert", instruction),
        );
      }
      BitTestType::FromRegister(location, target) => {
        assert_eq!(
          format!("B{}", reference_opcode.operands[0].name),
          location.to_string(),
          "{:?}",
          instruction
        );
        assert_operand(
          reference_opcode.operands[1].clone(),
          target.to_string(),
          format!("{:?} failed assert", instruction),
        );
      }
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
      match instruction {
        Instruction::NOP => {
          assert_eq!(reference_opcode.mnemonic, "NOP");
        }
        Instruction::INC(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "INC");
          match operation_type {
            IncDecOperationType::ToRegister(target) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            IncDecOperationType::ToAddress(target) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
          }
        }
        Instruction::AND(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "AND");
          assert_arithmetric(reference_opcode, operation_type, true, instruction);
        }
        Instruction::OR(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "OR");
          assert_arithmetric(reference_opcode, operation_type, true, instruction);
        }
        Instruction::CP(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "CP");
          assert_arithmetric(reference_opcode, operation_type, true, instruction);
        }
        Instruction::XOR(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "XOR");
          assert_arithmetric(reference_opcode, operation_type, true, instruction);
        }
        Instruction::ADD(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "ADD");
          assert_arithmetric(reference_opcode, operation_type, false, instruction);
        }
        Instruction::ADC(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "ADC");
          assert_arithmetric(reference_opcode, operation_type, false, instruction);
        }
        Instruction::SUB(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "SUB");
          assert_arithmetric(reference_opcode, operation_type, true, instruction);
        }
        Instruction::SBC(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "SBC");
          assert_arithmetric(reference_opcode, operation_type, false, instruction);
        }
        Instruction::DEC(operation_type) => {
          assert_eq!(reference_opcode.mnemonic, "DEC");
          match operation_type {
            IncDecOperationType::ToRegister(target) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            IncDecOperationType::ToAddress(target) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
          }
        }
        Instruction::LD(load_type) => {
          assert_eq!(reference_opcode.mnemonic, "LD");
          match load_type {
            LoadType::ToRegister(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            LoadType::ToAddress(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            LoadType::FromAddress(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            LoadType::ToOffsetAddress(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
          }
        }
        Instruction::LDH(load_type) => {
          assert_eq!(reference_opcode.mnemonic, "LDH");
          match load_type {
            LoadType::ToRegister(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            LoadType::ToAddress(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            LoadType::FromAddress(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
            LoadType::ToOffsetAddress(target, source) => {
              assert_operand(
                reference_opcode.operands[0].clone(),
                target.to_string(),
                format!("{:?} failed assert", instruction),
              );
              assert_operand(
                reference_opcode.operands[1].clone(),
                source.to_string(),
                format!("{:?} failed assert", instruction),
              );
            }
          }
        }
        Instruction::POP(target) => {
          assert_eq!(reference_opcode.mnemonic, "POP");
          assert_operand(
            reference_opcode.operands[0].clone(),
            target.to_string(),
            format!("{:?} failed assert", instruction),
          );
        }
        Instruction::PUSH(target) => {
          assert_eq!(reference_opcode.mnemonic, "PUSH");
          assert_operand(
            reference_opcode.operands[0].clone(),
            target.to_string(),
            format!("{:?} failed assert", instruction),
          );
        }
        Instruction::JR(condition, source) => {
          assert_eq!(reference_opcode.mnemonic, "JR");
          match condition {
            Condition::Always => {
              assert_eq!(
                reference_opcode.operands[0].name.to_uppercase(),
                source.to_string()
              );
            }
            Condition::NotZero => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "NZ");
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                source.to_string()
              );
            }
            Condition::Carry => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "C");
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                source.to_string()
              );
            }
            Condition::NotCarry => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "NC");
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                source.to_string()
              );
            }
            Condition::Zero => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "Z");
              assert_eq!(
                reference_opcode.operands[1].name.to_uppercase(),
                source.to_string()
              );
            }
          }
        }
        Instruction::CALL(condition, source) => {
          assert_eq!(reference_opcode.mnemonic, "CALL");
          assert_condition(
            reference_opcode,
            condition,
            source,
            format!("{:?} failed assert", instruction),
          );
        }
        Instruction::JP(condition, source) => {
          assert_eq!(reference_opcode.mnemonic, "JP");
          assert_condition(
            reference_opcode,
            condition,
            source,
            format!("{:?} failed assert", instruction),
          );
        }
        Instruction::RET(condition) => {
          assert_eq!(reference_opcode.mnemonic, "RET");
          match condition {
            Condition::NotZero => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "NZ");
            }
            Condition::Zero => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "Z");
            }
            Condition::Carry => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "C");
            }
            Condition::NotCarry => {
              assert_eq!(reference_opcode.operands[0].name.to_uppercase(), "NC");
            }
            Condition::Always => {
              assert_eq!(reference_opcode.operands.len(), 0);
            }
          }
        }
        Instruction::RST(location) => {
          assert_eq!(reference_opcode.mnemonic, "RST");
          assert_eq!(
            format!("X{}", reference_opcode.operands[0].name.to_uppercase()),
            location.to_string()
          );
        }
        Instruction::DI
        | Instruction::EI
        | Instruction::RLCA
        | Instruction::RRCA
        | Instruction::RLA
        | Instruction::RRA
        | Instruction::DAA
        | Instruction::CPL
        | Instruction::CCF
        | Instruction::STOP
        | Instruction::HALT
        | Instruction::PREFIX
        | Instruction::RETI
        | Instruction::SCF => {
          assert_eq!(reference_opcode.mnemonic, format!("{:?}", instruction));
        }
        Instruction::NAI => {
          assert_eq!(reference_opcode.mnemonic.contains("ILLEGAL"), true);
        }
        _ => {
          // Skipping rest since they are prefixed
        }
      }
    }
  }

  #[test]
  fn can_correctly_decode_prefix_opcode() {
    let mut file = File::open("res/opcodes.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let opcodes: Opcodes = serde_json::from_str(&contents).unwrap();

    for (opcode, reference_opcode) in opcodes.cbprefixed {
      let opcode_u8 = u8::from_str_radix(opcode.as_str().trim_start_matches("0x"), 16).unwrap();
      let instruction = Instruction::from_byte_prefixed(opcode_u8);
      if instruction != Instruction::NAI {
        print!("Checking: {:?}", instruction);
        match instruction {
          Instruction::RLC(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "RLC");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::RRC(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "RRC");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::RR(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "RR");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::RL(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "RL");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::SLA(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "SLA");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::SRA(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "SRA");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::SRL(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "SRL");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::SWAP(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "SWAP");
            assert_rotate(reference_opcode, operation_type, instruction);
          }
          Instruction::BIT(operation_type) => {
            assert_eq!(reference_opcode.mnemonic, "BIT");
            assert_bit_test(reference_opcode, operation_type, instruction);
          }
          _ => {
            // Skipping non prefixed instructions
          }
        }
        println!(" ✔");
      }
    }
  }
}
