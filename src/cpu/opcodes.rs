use crate::cpu::instruction::{
    AddressLocation, AddressSource, AddressTarget, ArithmeticOperationType, ArithmeticSource,
    ArithmeticTarget, Condition, ConditionSource, IncDecOperationType, IncDecTarget, Instruction,
    LoadSource, LoadType, RegisterSource, RegisterTarget,
};

/// SM83 non prefixed instruction map
///
pub static INSTRUCTION_MAP: [[Instruction; 0x10]; 0x10] = [
    // 0
    [
        Instruction::NOP,
        Instruction::LD(LoadType::ToRegister(RegisterTarget::BC, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::BC, LoadSource::A)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::BC)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::B)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::D8)),
        Instruction::RLCA,
        Instruction::LD(LoadType::ToAddress(AddressTarget::A16, LoadSource::SP)),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::HL,
            ArithmeticSource::BC,
        )),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::BC)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::BC)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::C)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::D8)),
        Instruction::RRCA,
    ],
    // 1
    [
        Instruction::STOP,
        Instruction::LD(LoadType::ToRegister(RegisterTarget::DE, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::DE, LoadSource::A)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::DE)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::D)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::D8)),
        Instruction::RLA,
        Instruction::JR(Condition::Always, ConditionSource::R8),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::HL,
            ArithmeticSource::DE,
        )),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::DE)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::DE)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::E)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::D8)),
        Instruction::RRA,
    ],
    // 2
    [
        Instruction::JR(Condition::NotZero, ConditionSource::R8),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::HL, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HLP, LoadSource::A)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::HL)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::H)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::D8)),
        Instruction::DAA,
        Instruction::JR(Condition::Zero, ConditionSource::R8),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::HL,
            ArithmeticSource::HL,
        )),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::HLP)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::HL)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::L)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::L)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::D8)),
        Instruction::CPL,
    ],
    // 3
    [
        Instruction::JR(Condition::NotCarry, ConditionSource::R8),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::SP, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HLM, LoadSource::A)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::SP)),
        Instruction::INC(IncDecOperationType::ToAddress(IncDecTarget::HL)),
        Instruction::DEC(IncDecOperationType::ToAddress(IncDecTarget::HL)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::D8)),
        Instruction::SCF,
        Instruction::JR(Condition::Carry, ConditionSource::R8),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::HL,
            ArithmeticSource::SP,
        )),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::HLM)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::SP)),
        Instruction::INC(IncDecOperationType::ToRegister(IncDecTarget::A)),
        Instruction::DEC(IncDecOperationType::ToRegister(IncDecTarget::A)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::D8)),
        Instruction::CCF,
    ],
    // 4
    [
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::B, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::B, LoadSource::A)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::C, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::C, LoadSource::A)),
    ],
    // 5
    [
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::D, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::D, LoadSource::A)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::E, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::E, LoadSource::A)),
    ],
    // 6
    [
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::H, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::H, LoadSource::A)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::L, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::L, LoadSource::A)),
    ],
    // 7
    [
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::B)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::C)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::D)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::E)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::H)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::L)),
        Instruction::HALT,
        Instruction::LD(LoadType::ToAddress(AddressTarget::HL, LoadSource::A)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::B)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::C)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::D)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::E)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::H)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::L)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::HL)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::A, LoadSource::A)),
    ],
    // 8
    [
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::ADD(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::ADC(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::ADC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
    ],
    // 9
    [
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::SUB(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::SBC(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
    ],
    // a
    [
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::AND(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::AND(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::XOR(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
    ],
    // b
    [
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::OR(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::B,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::C,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::E,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::H,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::L,
        )),
        Instruction::CP(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::HL,
        )),
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::A,
        )),
    ],
    // c
    [
        Instruction::RET(Condition::NotZero),
        Instruction::POP(RegisterTarget::BC),
        Instruction::JP(Condition::NotZero, ConditionSource::A16),
        Instruction::JP(Condition::Always, ConditionSource::A16),
        Instruction::CALL(Condition::NotZero, ConditionSource::A16),
        Instruction::PUSH(RegisterSource::BC),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X00H),
        Instruction::RET(Condition::Zero),
        Instruction::RET(Condition::Always),
        Instruction::JP(Condition::Zero, ConditionSource::A16),
        Instruction::PREFIX,
        Instruction::CALL(Condition::Zero, ConditionSource::A16),
        Instruction::CALL(Condition::Always, ConditionSource::A16),
        Instruction::ADC(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X08H),
    ],
    // d
    [
        Instruction::RET(Condition::NotCarry),
        Instruction::POP(RegisterTarget::DE),
        Instruction::JP(Condition::NotCarry, ConditionSource::A16),
        Instruction::NAI,
        Instruction::CALL(Condition::NotCarry, ConditionSource::A16),
        Instruction::PUSH(RegisterSource::DE),
        Instruction::SUB(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X10H),
        Instruction::RET(Condition::Carry),
        Instruction::RETI,
        Instruction::JP(Condition::Carry, ConditionSource::A16),
        Instruction::NAI,
        Instruction::CALL(Condition::Carry, ConditionSource::A16),
        Instruction::NAI,
        Instruction::SBC(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X18H),
    ],
    // e
    [
        Instruction::LDH(LoadType::ToOffsetAddress(AddressTarget::A8, LoadSource::A)),
        Instruction::POP(RegisterTarget::HL),
        Instruction::LD(LoadType::ToAddress(AddressTarget::C, LoadSource::A)),
        Instruction::NAI,
        Instruction::NAI,
        Instruction::PUSH(RegisterSource::HL),
        Instruction::AND(ArithmeticOperationType::FromAddress(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X20H),
        Instruction::ADD(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::SP,
            ArithmeticSource::R8,
        )),
        Instruction::JP(Condition::Always, ConditionSource::HL),
        Instruction::LD(LoadType::ToAddress(AddressTarget::A16, LoadSource::A)),
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::XOR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X28H),
    ],
    // f
    [
        Instruction::LDH(LoadType::FromAddress(RegisterTarget::A, AddressSource::A8)),
        Instruction::POP(RegisterTarget::AF),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::C)),
        Instruction::DI,
        Instruction::NAI,
        Instruction::PUSH(RegisterSource::AF),
        Instruction::OR(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X30H),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::HL, LoadSource::SPP)),
        Instruction::LD(LoadType::ToRegister(RegisterTarget::SP, LoadSource::HL)),
        Instruction::LD(LoadType::FromAddress(RegisterTarget::A, AddressSource::A16)),
        Instruction::EI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::CP(ArithmeticOperationType::ToRegister(
            ArithmeticTarget::A,
            ArithmeticSource::D8,
        )),
        Instruction::RST(AddressLocation::X38H),
    ],
];

/// SM83 prefixed instruction map
///
pub static PREFIX_INSTRUCTION_MAP: [[Instruction; 0x10]; 0x10] = [
    // 0
    [
        Instruction::NOP,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 1
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 2
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 3
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 4
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 5
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 6
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 7
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 8
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // 9
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // a
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // b
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // c
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // d
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // e
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
    // f
    [
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
        Instruction::NAI,
    ],
];
