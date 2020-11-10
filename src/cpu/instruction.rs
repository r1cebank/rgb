use super::opcodes;
use strum_macros::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    INC(TargetType),
    DEC(TargetType),
    OR(SourceType),
    CP(OperationType),
    ADD(OperationType),
    AND(OperationType),
    SUB(OperationType),
    ADC(OperationType),
    SBC(OperationType),
    XOR(SourceType),
    JR(Condition, Address),
    JP(Condition, Address),
    CALL(Condition, Address),
    RST(AddressLocation),
    PUSH(Register),
    POP(Register),
    RET(Condition),
    LD(OperationType),
    LDH(OperationType),
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
    RL(TargetType),
    RR(TargetType),
    RLC(TargetType),
    RRC(TargetType),
    SLA(TargetType),
    SRA(TargetType),
    SRL(TargetType),
    SWAP(TargetType),
    BIT(TargetType, BitLocation),
    RES(TargetType, BitLocation),
    SET(TargetType, BitLocation),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum TargetType {
    Address(Address),
    Register(Register),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum SourceType {
    Address(Address),
    Register(Register),
    Value(Value),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum OperationType {
    RegisterToRegister(Register, Register),
    RegisterToAddress(Address, Register),
    AddressToAddress(Address, Address),
    AddressToRegister(Register, Address),
    ValueToRegister(Register, Value),
    ValueToAddress(Address, Value),
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum Register {
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
    SPP,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum Value {
    D8,
    D16,
    R8,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum Address {
    C,
    HL,
    HLP,
    HLM,
    R8,
    A8,
    A16,
    BC,
    DE,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum BitLocation {
    B0 = 0,
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
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
        address: Address,
        custom_msg: String,
    ) {
        match condition {
            Condition::Always => {
                assert_eq!(
                    reference_opcode.operands[0].name.to_uppercase(),
                    address.to_string(),
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
                    address.to_string(),
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
                    address.to_string(),
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
                    address.to_string(),
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
                    address.to_string(),
                    "{}",
                    custom_msg
                );
            }
        }
    }

    fn assert_operation(
        reference_opcode: Opcode,
        operation_type: OperationType,
        fixed_target: bool,
        instruction: Instruction,
    ) {
        if fixed_target {
            match operation_type {
                OperationType::RegisterToAddress(target, source) => {
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
                    assert_eq!(
                        reference_opcode.operands[0].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::AddressToRegister(target, source) => {
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
                    assert_eq!(
                        reference_opcode.operands[0].immediate, false,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::RegisterToRegister(target, source) => {
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
                    assert_eq!(
                        reference_opcode.operands[0].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::ValueToAddress(target, source) => {
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
                    assert_eq!(
                        reference_opcode.operands[0].immediate, false,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::ValueToRegister(target, source) => {
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
                    assert_eq!(
                        reference_opcode.operands[0].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::AddressToAddress(_, _) => {
                    panic!("This is not supposed to happen {:?}", instruction);
                }
            }
        } else {
            match operation_type {
                OperationType::RegisterToAddress(target, source) => {
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        target.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                    assert_eq!(
                        reference_opcode.operands[0].immediate, false,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_eq!(
                        reference_opcode.operands[1].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[1].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::AddressToRegister(target, source) => {
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        target.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                    assert_eq!(
                        reference_opcode.operands[0].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_eq!(
                        reference_opcode.operands[1].immediate, false,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[1].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::RegisterToRegister(target, source) => {
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        target.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                    assert_eq!(
                        reference_opcode.operands[0].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_eq!(
                        reference_opcode.operands[1].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[1].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::ValueToAddress(target, source) => {
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        target.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                    assert_eq!(
                        reference_opcode.operands[0].immediate, false,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[1].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::ValueToRegister(target, source) => {
                    assert_operand(
                        reference_opcode.operands[0].clone(),
                        target.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                    assert_eq!(
                        reference_opcode.operands[0].immediate, true,
                        "{:?} failed assert",
                        instruction
                    );
                    assert_operand(
                        reference_opcode.operands[1].clone(),
                        source.to_string(),
                        format!("{:?} failed assert", instruction),
                    );
                }
                OperationType::AddressToAddress(_, _) => {
                    panic!("This is not supposed to happen {:?}", instruction);
                }
            }
        }
    }

    fn assert_target(
        reference_opcode: Opcode,
        operation_type: TargetType,
        instruction: Instruction,
    ) {
        match operation_type {
            TargetType::Address(target) => {
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
            TargetType::Register(target) => {
                assert_operand(
                    reference_opcode.operands[0].clone(),
                    target.to_string(),
                    format!("{:?} failed assert", instruction),
                );
            }
        }
    }

    fn assert_source(reference_opcode: Opcode, source_type: SourceType, instruction: Instruction) {
        match source_type {
            SourceType::Address(target) => {
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
            SourceType::Register(target) => {
                assert_operand(
                    reference_opcode.operands[0].clone(),
                    target.to_string(),
                    format!("{:?} failed assert", instruction),
                );
            }
            SourceType::Value(value) => {
                assert_operand(
                    reference_opcode.operands[0].clone(),
                    value.to_string(),
                    format!("{:?} failed assert", instruction),
                );
            }
        }
    }

    fn assert_target_and_location(
        reference_opcode: Opcode,
        operation_type: TargetType,
        location: BitLocation,
        instruction: Instruction,
    ) {
        match operation_type {
            TargetType::Address(target) => {
                assert_eq!(
                    reference_opcode.operands[1].immediate, false,
                    "{:?}",
                    instruction
                );
                assert_operand(
                    reference_opcode.operands[0].clone(),
                    String::from(location.to_string().as_str().trim_start_matches("B")),
                    format!("{:?} failed assert", instruction),
                );
                assert_operand(
                    reference_opcode.operands[1].clone(),
                    target.to_string(),
                    format!("{:?} failed assert", instruction),
                );
            }
            TargetType::Register(target) => {
                assert_operand(
                    reference_opcode.operands[0].clone(),
                    String::from(location.to_string().as_str().trim_start_matches("B")),
                    format!("{:?} failed assert", instruction),
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
            let opcode_u8 =
                u8::from_str_radix(opcode.as_str().trim_start_matches("0x"), 16).unwrap();
            let instruction = Instruction::from_byte(opcode_u8, false);
            match instruction {
                Instruction::NOP => {
                    assert_eq!(reference_opcode.mnemonic, "NOP");
                }
                Instruction::DEC(target_type) => {
                    assert_eq!(reference_opcode.mnemonic, "DEC");
                    assert_target(reference_opcode, target_type, instruction);
                }
                Instruction::INC(target_type) => {
                    assert_eq!(reference_opcode.mnemonic, "INC");
                    assert_target(reference_opcode, target_type, instruction);
                }
                Instruction::AND(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "AND");
                    assert_operation(reference_opcode, operation_type, true, instruction);
                }
                Instruction::OR(source_type) => {
                    assert_eq!(reference_opcode.mnemonic, "OR");
                    assert_source(reference_opcode, source_type, instruction);
                }
                Instruction::CP(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "CP");
                    assert_operation(reference_opcode, operation_type, true, instruction);
                }
                Instruction::XOR(source_type) => {
                    assert_eq!(reference_opcode.mnemonic, "XOR");
                    assert_source(reference_opcode, source_type, instruction);
                }
                Instruction::ADD(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "ADD");
                    assert_operation(reference_opcode, operation_type, false, instruction);
                }
                Instruction::ADC(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "ADC");
                    assert_operation(reference_opcode, operation_type, false, instruction);
                }
                Instruction::SUB(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "SUB");
                    assert_operation(reference_opcode, operation_type, true, instruction);
                }
                Instruction::SBC(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "SBC");
                    assert_operation(reference_opcode, operation_type, false, instruction);
                }
                Instruction::LD(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "LD");
                    assert_operation(reference_opcode, operation_type, false, instruction);
                }
                Instruction::LDH(operation_type) => {
                    assert_eq!(reference_opcode.mnemonic, "LDH");
                    assert_operation(reference_opcode, operation_type, false, instruction);
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
                    panic!("Should never reach here: {:?}", instruction);
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
            let opcode_u8 =
                u8::from_str_radix(opcode.as_str().trim_start_matches("0x"), 16).unwrap();
            let instruction = Instruction::from_byte(opcode_u8, true);
            match instruction {
                Instruction::RLC(target) => {
                    assert_eq!(reference_opcode.mnemonic, "RLC");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::RRC(target) => {
                    assert_eq!(reference_opcode.mnemonic, "RRC");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::RR(target) => {
                    assert_eq!(reference_opcode.mnemonic, "RR");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::RL(target) => {
                    assert_eq!(reference_opcode.mnemonic, "RL");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::SLA(target) => {
                    assert_eq!(reference_opcode.mnemonic, "SLA");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::SRA(target) => {
                    assert_eq!(reference_opcode.mnemonic, "SRA");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::SRL(target) => {
                    assert_eq!(reference_opcode.mnemonic, "SRL");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::SWAP(target) => {
                    assert_eq!(reference_opcode.mnemonic, "SWAP");
                    assert_target(reference_opcode, target, instruction);
                }
                Instruction::BIT(target, location) => {
                    assert_eq!(reference_opcode.mnemonic, "BIT");
                    assert_target_and_location(reference_opcode, target, location, instruction);
                }
                Instruction::RES(target, location) => {
                    assert_eq!(reference_opcode.mnemonic, "RES");
                    assert_target_and_location(reference_opcode, target, location, instruction);
                }
                Instruction::SET(target, location) => {
                    assert_eq!(reference_opcode.mnemonic, "SET");
                    assert_target_and_location(reference_opcode, target, location, instruction);
                }
                _ => {
                    // Skipping non prefixed instructions
                    panic!("Should never reach here: {:?}", instruction);
                }
            }
        }
    }
}
