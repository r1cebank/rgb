pub enum Instruction {
  ADD(ArithmeticTarget),
  INC(IncDecTarget),
  DEC(IncDecTarget),
  JP(JumpCondition),
}

pub enum IncDecTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  HLI,
  HL,
  BC,
  DE,
  SP,
}

pub enum JumpCondition {
  NotZero,
  Zero,
  NotCarry,
  Carry,
  Always,
}

pub enum ArithmeticTarget {
  A,
  B,
  C,
  D,
  E,
  H,
  L,
  HLI,
  D8,
}

impl Instruction {
  pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      _ =>
      /* TODO: Add mapping for rest of instructions */
      {
        None
      }
    }
  }

  pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      // INC xx
      0x3c => Some(Instruction::INC(IncDecTarget::A)),
      0x04 => Some(Instruction::INC(IncDecTarget::B)),
      0x0c => Some(Instruction::INC(IncDecTarget::C)),
      0x14 => Some(Instruction::INC(IncDecTarget::D)),
      0x1c => Some(Instruction::INC(IncDecTarget::E)),
      0x24 => Some(Instruction::INC(IncDecTarget::H)),
      0x2c => Some(Instruction::INC(IncDecTarget::L)),
      0x23 => Some(Instruction::INC(IncDecTarget::HL)),
      0x34 => Some(Instruction::INC(IncDecTarget::HLI)),
      0x03 => Some(Instruction::INC(IncDecTarget::BC)),
      0x13 => Some(Instruction::INC(IncDecTarget::DE)),
      0x33 => Some(Instruction::INC(IncDecTarget::SP)),

      // DEC xx
      0x3d => Some(Instruction::DEC(IncDecTarget::A)),
      0x05 => Some(Instruction::DEC(IncDecTarget::B)),
      0x0d => Some(Instruction::DEC(IncDecTarget::C)),
      0x15 => Some(Instruction::DEC(IncDecTarget::D)),
      0x1d => Some(Instruction::DEC(IncDecTarget::E)),
      0x25 => Some(Instruction::DEC(IncDecTarget::H)),
      0x2d => Some(Instruction::DEC(IncDecTarget::L)),
      0x2b => Some(Instruction::DEC(IncDecTarget::HL)),
      0x35 => Some(Instruction::DEC(IncDecTarget::HLI)),
      0x0b => Some(Instruction::DEC(IncDecTarget::BC)),
      0x1b => Some(Instruction::DEC(IncDecTarget::DE)),
      0x3b => Some(Instruction::DEC(IncDecTarget::SP)),
      _ =>
      /* TODO: Add mapping for rest of instructions */
      {
        None
      }
    }
  }
}
