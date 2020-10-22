use crate::cpu::instruction::{
    AddressSource, AddressTarget, ArithmeticSource, ArithmeticTarget, IncDecTarget, Instruction,
    JumpCondition, JumpSource, LoadSource, LoadTarget, LoadType,
};

/// SM83 non prefixed instruction map
///
pub static INSTRUCTION_MAP: [[Instruction; 0x10]; 0x10] = [
    // 0
    [
        Instruction::NOP,
        Instruction::LD(LoadType::ToRegister(LoadTarget::BC, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::BC, LoadSource::A)),
        Instruction::INC(IncDecTarget::BC),
        Instruction::INC(IncDecTarget::B),
        Instruction::DEC(IncDecTarget::B),
        Instruction::LD(LoadType::ToRegister(LoadTarget::B, LoadSource::D8)),
        Instruction::RLCA,
        Instruction::LD(LoadType::ToAddress(AddressTarget::A16, LoadSource::SP)),
        Instruction::ADD(ArithmeticTarget::HL, ArithmeticSource::BC),
        Instruction::LD(LoadType::FromAddress(LoadTarget::A, AddressSource::BC)),
        Instruction::DEC(IncDecTarget::BC),
        Instruction::INC(IncDecTarget::C),
        Instruction::DEC(IncDecTarget::C),
        Instruction::LD(LoadType::ToRegister(LoadTarget::C, LoadSource::D8)),
        Instruction::RRCA,
    ],
    // 1
    [
        Instruction::STOP,
        Instruction::LD(LoadType::ToRegister(LoadTarget::DE, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::DE, LoadSource::A)),
        Instruction::INC(IncDecTarget::DE),
        Instruction::INC(IncDecTarget::D),
        Instruction::DEC(IncDecTarget::D),
        Instruction::LD(LoadType::ToRegister(LoadTarget::D, LoadSource::D8)),
        Instruction::RLA,
        Instruction::JR(JumpCondition::Always, JumpSource::R8),
        Instruction::ADD(ArithmeticTarget::HL, ArithmeticSource::DE),
        Instruction::LD(LoadType::FromAddress(LoadTarget::A, AddressSource::DE)),
        Instruction::DEC(IncDecTarget::DE),
        Instruction::INC(IncDecTarget::E),
        Instruction::DEC(IncDecTarget::E),
        Instruction::LD(LoadType::ToRegister(LoadTarget::E, LoadSource::D8)),
        Instruction::RRA,
    ],
    // 2
    [
        Instruction::JR(JumpCondition::NotZero, JumpSource::R8),
        Instruction::LD(LoadType::ToRegister(LoadTarget::HL, LoadSource::D16)),
        Instruction::LD(LoadType::ToAddress(AddressTarget::HLP, LoadSource::A)),
        Instruction::INC(IncDecTarget::HL),
        Instruction::INC(IncDecTarget::H),
        Instruction::DEC(IncDecTarget::H),
        Instruction::LD(LoadType::ToRegister(LoadTarget::H, LoadSource::D8)),
        Instruction::DAA,
        Instruction::JR(JumpCondition::Zero, JumpSource::R8),
        Instruction::ADD(ArithmeticTarget::HL, ArithmeticSource::HL),
        Instruction::LD(LoadType::FromAddress(LoadTarget::A, AddressSource::HLP)),
        Instruction::DEC(IncDecTarget::HL),
        Instruction::INC(IncDecTarget::L),
        Instruction::DEC(IncDecTarget::L),
        Instruction::LD(LoadType::ToRegister(LoadTarget::L, LoadSource::D8)),
        Instruction::CPL,
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
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::B),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::C),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::D),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::E),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::H),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::L),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::HLI),
        Instruction::ADD(ArithmeticTarget::A, ArithmeticSource::A),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::B),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::C),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::D),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::E),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::H),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::L),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::HLI),
        Instruction::ADC(ArithmeticTarget::A, ArithmeticSource::A),
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
