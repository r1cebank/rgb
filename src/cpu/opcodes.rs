use crate::cpu::instruction::{
    Address, AddressLocation, BitLocation, Condition, Instruction, OperationType, Register,
    SourceType, TargetType, Value,
};

/// SM83 non prefixed instruction map
///
pub static INSTRUCTION_MAP: [[Instruction; 0x10]; 0x10] = [
    // 0
    [
        Instruction::NOP,
        Instruction::LD(OperationType::ValueToRegister(Register::BC, Value::D16)),
        Instruction::LD(OperationType::RegisterToAddress(Address::BC, Register::A)),
        Instruction::INC(TargetType::Register(Register::BC)),
        Instruction::INC(TargetType::Register(Register::B)),
        Instruction::DEC(TargetType::Register(Register::B)),
        Instruction::LD(OperationType::ValueToRegister(Register::B, Value::D8)),
        Instruction::RLCA,
        Instruction::LD(OperationType::RegisterToAddress(Address::A16, Register::SP)),
        Instruction::ADD(OperationType::RegisterToRegister(
            Register::HL,
            Register::BC,
        )),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::BC)),
        Instruction::DEC(TargetType::Register(Register::BC)),
        Instruction::INC(TargetType::Register(Register::C)),
        Instruction::DEC(TargetType::Register(Register::C)),
        Instruction::LD(OperationType::ValueToRegister(Register::C, Value::D8)),
        Instruction::RRCA,
    ],
    // 1
    [
        Instruction::STOP,
        Instruction::LD(OperationType::ValueToRegister(Register::DE, Value::D16)),
        Instruction::LD(OperationType::RegisterToAddress(Address::DE, Register::A)),
        Instruction::INC(TargetType::Register(Register::DE)),
        Instruction::INC(TargetType::Register(Register::D)),
        Instruction::DEC(TargetType::Register(Register::D)),
        Instruction::LD(OperationType::ValueToRegister(Register::D, Value::D8)),
        Instruction::RLA,
        Instruction::JR(Condition::Always, Address::R8),
        Instruction::ADD(OperationType::RegisterToRegister(
            Register::HL,
            Register::DE,
        )),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::DE)),
        Instruction::DEC(TargetType::Register(Register::DE)),
        Instruction::INC(TargetType::Register(Register::E)),
        Instruction::DEC(TargetType::Register(Register::E)),
        Instruction::LD(OperationType::ValueToRegister(Register::E, Value::D8)),
        Instruction::RRA,
    ],
    // 2
    [
        Instruction::JR(Condition::NotZero, Address::R8),
        Instruction::LD(OperationType::ValueToRegister(Register::HL, Value::D16)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HLP, Register::A)),
        Instruction::INC(TargetType::Register(Register::HL)),
        Instruction::INC(TargetType::Register(Register::H)),
        Instruction::DEC(TargetType::Register(Register::H)),
        Instruction::LD(OperationType::ValueToRegister(Register::H, Value::D8)),
        Instruction::DAA,
        Instruction::JR(Condition::Zero, Address::R8),
        Instruction::ADD(OperationType::RegisterToRegister(
            Register::HL,
            Register::HL,
        )),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::HLP)),
        Instruction::DEC(TargetType::Register(Register::HL)),
        Instruction::INC(TargetType::Register(Register::L)),
        Instruction::DEC(TargetType::Register(Register::L)),
        Instruction::LD(OperationType::ValueToRegister(Register::L, Value::D8)),
        Instruction::CPL,
    ],
    // 3
    [
        Instruction::JR(Condition::NotCarry, Address::R8),
        Instruction::LD(OperationType::ValueToRegister(Register::SP, Value::D16)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HLM, Register::A)),
        Instruction::INC(TargetType::Register(Register::SP)),
        Instruction::INC(TargetType::Address(Address::HL)),
        Instruction::DEC(TargetType::Address(Address::HL)),
        Instruction::LD(OperationType::ValueToAddress(Address::HL, Value::D8)),
        Instruction::SCF,
        Instruction::JR(Condition::Carry, Address::R8),
        Instruction::ADD(OperationType::RegisterToRegister(
            Register::HL,
            Register::SP,
        )),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::HLM)),
        Instruction::DEC(TargetType::Register(Register::SP)),
        Instruction::INC(TargetType::Register(Register::A)),
        Instruction::DEC(TargetType::Register(Register::A)),
        Instruction::LD(OperationType::ValueToRegister(Register::A, Value::D8)),
        Instruction::CCF,
    ],
    // 4
    [
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::B, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::B, Register::A)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::C, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::C, Register::A)),
    ],
    // 5
    [
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::D, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::D, Register::A)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::E, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::E, Register::A)),
    ],
    // 6
    [
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::H, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::H, Register::A)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::L, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::L, Register::A)),
    ],
    // 7
    [
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::B)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::C)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::D)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::E)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::H)),
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::L)),
        Instruction::HALT,
        Instruction::LD(OperationType::RegisterToAddress(Address::HL, Register::A)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::B)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::C)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::D)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::E)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::H)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::L)),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::HL)),
        Instruction::LD(OperationType::RegisterToRegister(Register::A, Register::A)),
    ],
    // 8
    [
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::B)),
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::C)),
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::D)),
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::E)),
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::H)),
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::L)),
        Instruction::ADD(OperationType::AddressToRegister(Register::A, Address::HL)),
        Instruction::ADD(OperationType::RegisterToRegister(Register::A, Register::A)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::B)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::C)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::D)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::E)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::H)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::L)),
        Instruction::ADC(OperationType::AddressToRegister(Register::A, Address::HL)),
        Instruction::ADC(OperationType::RegisterToRegister(Register::A, Register::A)),
    ],
    // 9
    [
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::B)),
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::C)),
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::D)),
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::E)),
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::H)),
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::L)),
        Instruction::SUB(OperationType::AddressToRegister(Register::A, Address::HL)),
        Instruction::SUB(OperationType::RegisterToRegister(Register::A, Register::A)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::B)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::C)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::D)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::E)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::H)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::L)),
        Instruction::SBC(OperationType::AddressToRegister(Register::A, Address::HL)),
        Instruction::SBC(OperationType::RegisterToRegister(Register::A, Register::A)),
    ],
    // a
    [
        Instruction::AND(SourceType::Register(Register::B)),
        Instruction::AND(SourceType::Register(Register::C)),
        Instruction::AND(SourceType::Register(Register::D)),
        Instruction::AND(SourceType::Register(Register::E)),
        Instruction::AND(SourceType::Register(Register::H)),
        Instruction::AND(SourceType::Register(Register::L)),
        Instruction::AND(SourceType::Address(Address::HL)),
        Instruction::AND(SourceType::Register(Register::A)),
        Instruction::XOR(SourceType::Register(Register::B)),
        Instruction::XOR(SourceType::Register(Register::C)),
        Instruction::XOR(SourceType::Register(Register::D)),
        Instruction::XOR(SourceType::Register(Register::E)),
        Instruction::XOR(SourceType::Register(Register::H)),
        Instruction::XOR(SourceType::Register(Register::L)),
        Instruction::XOR(SourceType::Address(Address::HL)),
        Instruction::XOR(SourceType::Register(Register::A)),
    ],
    // b
    [
        Instruction::OR(SourceType::Register(Register::B)),
        Instruction::OR(SourceType::Register(Register::C)),
        Instruction::OR(SourceType::Register(Register::D)),
        Instruction::OR(SourceType::Register(Register::E)),
        Instruction::OR(SourceType::Register(Register::H)),
        Instruction::OR(SourceType::Register(Register::L)),
        Instruction::OR(SourceType::Address(Address::HL)),
        Instruction::OR(SourceType::Register(Register::A)),
        Instruction::CP(SourceType::Register(Register::B)),
        Instruction::CP(SourceType::Register(Register::C)),
        Instruction::CP(SourceType::Register(Register::D)),
        Instruction::CP(SourceType::Register(Register::E)),
        Instruction::CP(SourceType::Register(Register::H)),
        Instruction::CP(SourceType::Register(Register::L)),
        Instruction::CP(SourceType::Address(Address::HL)),
        Instruction::CP(SourceType::Register(Register::A)),
    ],
    // c
    [
        Instruction::RET(Condition::NotZero),
        Instruction::POP(Register::BC),
        Instruction::JP(Condition::NotZero, Address::A16),
        Instruction::JP(Condition::Always, Address::A16),
        Instruction::CALL(Condition::NotZero, Address::A16),
        Instruction::PUSH(Register::BC),
        Instruction::ADD(OperationType::ValueToRegister(Register::A, Value::D8)),
        Instruction::RST(AddressLocation::X00H),
        Instruction::RET(Condition::Zero),
        Instruction::RET(Condition::Always),
        Instruction::JP(Condition::Zero, Address::A16),
        Instruction::PREFIX,
        Instruction::CALL(Condition::Zero, Address::A16),
        Instruction::CALL(Condition::Always, Address::A16),
        Instruction::ADC(OperationType::ValueToRegister(Register::A, Value::D8)),
        Instruction::RST(AddressLocation::X08H),
    ],
    // d
    [
        Instruction::RET(Condition::NotCarry),
        Instruction::POP(Register::DE),
        Instruction::JP(Condition::NotCarry, Address::A16),
        Instruction::NAI,
        Instruction::CALL(Condition::NotCarry, Address::A16),
        Instruction::PUSH(Register::DE),
        Instruction::SUB(OperationType::ValueToRegister(Register::A, Value::D8)),
        Instruction::RST(AddressLocation::X10H),
        Instruction::RET(Condition::Carry),
        Instruction::RETI,
        Instruction::JP(Condition::Carry, Address::A16),
        Instruction::NAI,
        Instruction::CALL(Condition::Carry, Address::A16),
        Instruction::NAI,
        Instruction::SBC(OperationType::ValueToRegister(Register::A, Value::D8)),
        Instruction::RST(AddressLocation::X18H),
    ],
    // e
    [
        Instruction::LDH(OperationType::RegisterToAddress(Address::A8, Register::A)),
        Instruction::POP(Register::HL),
        Instruction::LD(OperationType::RegisterToAddress(Address::C, Register::A)),
        Instruction::NAI,
        Instruction::NAI,
        Instruction::PUSH(Register::HL),
        Instruction::AND(SourceType::Value(Value::D8)),
        Instruction::RST(AddressLocation::X20H),
        Instruction::ADD(OperationType::ValueToRegister(Register::SP, Value::R8)),
        Instruction::JP(Condition::Always, Address::HL),
        Instruction::LD(OperationType::RegisterToAddress(Address::A16, Register::A)),
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::XOR(SourceType::Value(Value::D8)),
        Instruction::RST(AddressLocation::X28H),
    ],
    // f
    [
        Instruction::LDH(OperationType::AddressToRegister(Register::A, Address::A8)),
        Instruction::POP(Register::AF),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::C)),
        Instruction::DI,
        Instruction::NAI,
        Instruction::PUSH(Register::AF),
        Instruction::OR(SourceType::Value(Value::D8)),
        Instruction::RST(AddressLocation::X30H),
        Instruction::LD(OperationType::RegisterToRegister(
            Register::HL,
            Register::SPP,
        )),
        Instruction::LD(OperationType::RegisterToRegister(
            Register::SP,
            Register::HL,
        )),
        Instruction::LD(OperationType::AddressToRegister(Register::A, Address::A16)),
        Instruction::EI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::CP(SourceType::Value(Value::D8)),
        Instruction::RST(AddressLocation::X38H),
    ],
];

/// SM83 prefixed instruction map
///
pub static PREFIX_INSTRUCTION_MAP: [[Instruction; 0x10]; 0x10] = [
    // 0
    [
        Instruction::RLC(TargetType::Register(Register::B)),
        Instruction::RLC(TargetType::Register(Register::C)),
        Instruction::RLC(TargetType::Register(Register::D)),
        Instruction::RLC(TargetType::Register(Register::E)),
        Instruction::RLC(TargetType::Register(Register::H)),
        Instruction::RLC(TargetType::Register(Register::L)),
        Instruction::RLC(TargetType::Address(Address::HL)),
        Instruction::RLC(TargetType::Register(Register::A)),
        Instruction::RRC(TargetType::Register(Register::B)),
        Instruction::RRC(TargetType::Register(Register::C)),
        Instruction::RRC(TargetType::Register(Register::D)),
        Instruction::RRC(TargetType::Register(Register::E)),
        Instruction::RRC(TargetType::Register(Register::H)),
        Instruction::RRC(TargetType::Register(Register::L)),
        Instruction::RRC(TargetType::Address(Address::HL)),
        Instruction::RRC(TargetType::Register(Register::A)),
    ],
    // 1
    [
        Instruction::RL(TargetType::Register(Register::B)),
        Instruction::RL(TargetType::Register(Register::C)),
        Instruction::RL(TargetType::Register(Register::D)),
        Instruction::RL(TargetType::Register(Register::E)),
        Instruction::RL(TargetType::Register(Register::H)),
        Instruction::RL(TargetType::Register(Register::L)),
        Instruction::RL(TargetType::Address(Address::HL)),
        Instruction::RL(TargetType::Register(Register::A)),
        Instruction::RR(TargetType::Register(Register::B)),
        Instruction::RR(TargetType::Register(Register::C)),
        Instruction::RR(TargetType::Register(Register::D)),
        Instruction::RR(TargetType::Register(Register::E)),
        Instruction::RR(TargetType::Register(Register::H)),
        Instruction::RR(TargetType::Register(Register::L)),
        Instruction::RR(TargetType::Address(Address::HL)),
        Instruction::RR(TargetType::Register(Register::A)),
    ],
    // 2
    [
        Instruction::SLA(TargetType::Register(Register::B)),
        Instruction::SLA(TargetType::Register(Register::C)),
        Instruction::SLA(TargetType::Register(Register::D)),
        Instruction::SLA(TargetType::Register(Register::E)),
        Instruction::SLA(TargetType::Register(Register::H)),
        Instruction::SLA(TargetType::Register(Register::L)),
        Instruction::SLA(TargetType::Address(Address::HL)),
        Instruction::SLA(TargetType::Register(Register::A)),
        Instruction::SRA(TargetType::Register(Register::B)),
        Instruction::SRA(TargetType::Register(Register::C)),
        Instruction::SRA(TargetType::Register(Register::D)),
        Instruction::SRA(TargetType::Register(Register::E)),
        Instruction::SRA(TargetType::Register(Register::H)),
        Instruction::SRA(TargetType::Register(Register::L)),
        Instruction::SRA(TargetType::Address(Address::HL)),
        Instruction::SRA(TargetType::Register(Register::A)),
    ],
    // 3
    [
        Instruction::SWAP(TargetType::Register(Register::B)),
        Instruction::SWAP(TargetType::Register(Register::C)),
        Instruction::SWAP(TargetType::Register(Register::D)),
        Instruction::SWAP(TargetType::Register(Register::E)),
        Instruction::SWAP(TargetType::Register(Register::H)),
        Instruction::SWAP(TargetType::Register(Register::L)),
        Instruction::SWAP(TargetType::Address(Address::HL)),
        Instruction::SWAP(TargetType::Register(Register::A)),
        Instruction::SRL(TargetType::Register(Register::B)),
        Instruction::SRL(TargetType::Register(Register::C)),
        Instruction::SRL(TargetType::Register(Register::D)),
        Instruction::SRL(TargetType::Register(Register::E)),
        Instruction::SRL(TargetType::Register(Register::H)),
        Instruction::SRL(TargetType::Register(Register::L)),
        Instruction::SRL(TargetType::Address(Address::HL)),
        Instruction::SRL(TargetType::Register(Register::A)),
    ],
    // 4
    [
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B0),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B0),
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B1),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B1),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B1),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B1),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B1),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B1),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B1),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B1),
    ],
    // 5
    [
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B2),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B2),
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B3),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B3),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B3),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B3),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B3),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B3),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B3),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B3),
    ],
    // 6
    [
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B4),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B4),
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B5),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B5),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B5),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B5),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B5),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B5),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B5),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B5),
    ],
    // 7
    [
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B6),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B6),
        Instruction::BIT(TargetType::Register(Register::B), BitLocation::B7),
        Instruction::BIT(TargetType::Register(Register::C), BitLocation::B7),
        Instruction::BIT(TargetType::Register(Register::D), BitLocation::B7),
        Instruction::BIT(TargetType::Register(Register::E), BitLocation::B7),
        Instruction::BIT(TargetType::Register(Register::H), BitLocation::B7),
        Instruction::BIT(TargetType::Register(Register::L), BitLocation::B7),
        Instruction::BIT(TargetType::Address(Address::HL), BitLocation::B7),
        Instruction::BIT(TargetType::Register(Register::A), BitLocation::B7),
    ],
    // 8
    [
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B0),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B0),
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B1),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B1),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B1),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B1),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B1),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B1),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B1),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B1),
    ],
    // 9
    [
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B2),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B2),
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B3),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B3),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B3),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B3),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B3),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B3),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B3),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B3),
    ],
    // a
    [
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B4),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B4),
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B5),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B5),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B5),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B5),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B5),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B5),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B5),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B5),
    ],
    // b
    [
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B6),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B6),
        Instruction::RES(TargetType::Register(Register::B), BitLocation::B7),
        Instruction::RES(TargetType::Register(Register::C), BitLocation::B7),
        Instruction::RES(TargetType::Register(Register::D), BitLocation::B7),
        Instruction::RES(TargetType::Register(Register::E), BitLocation::B7),
        Instruction::RES(TargetType::Register(Register::H), BitLocation::B7),
        Instruction::RES(TargetType::Register(Register::L), BitLocation::B7),
        Instruction::RES(TargetType::Address(Address::HL), BitLocation::B7),
        Instruction::RES(TargetType::Register(Register::A), BitLocation::B7),
    ],
    // c
    [
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B0),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B0),
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B1),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B1),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B1),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B1),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B1),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B1),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B1),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B1),
    ],
    // d
    [
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B2),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B2),
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B3),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B3),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B3),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B3),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B3),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B3),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B3),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B3),
    ],
    // e
    [
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B4),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B4),
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B5),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B5),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B5),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B5),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B5),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B5),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B5),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B5),
    ],
    // f
    [
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B6),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B6),
        Instruction::SET(TargetType::Register(Register::B), BitLocation::B7),
        Instruction::SET(TargetType::Register(Register::C), BitLocation::B7),
        Instruction::SET(TargetType::Register(Register::D), BitLocation::B7),
        Instruction::SET(TargetType::Register(Register::E), BitLocation::B7),
        Instruction::SET(TargetType::Register(Register::H), BitLocation::B7),
        Instruction::SET(TargetType::Register(Register::L), BitLocation::B7),
        Instruction::SET(TargetType::Address(Address::HL), BitLocation::B7),
        Instruction::SET(TargetType::Register(Register::A), BitLocation::B7),
    ],
];
