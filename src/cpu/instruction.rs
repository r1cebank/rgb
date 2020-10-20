pub enum Instruction {
  ADD(ArithmeticTarget),
  INC(IncDecTarget),
  JP(JumpCondition),
}

pub enum IncDecTarget {
  BC,
  DE,
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
      0x03 => Some(Instruction::INC(IncDecTarget::BC)),
      _ =>
      /* TODO: Add mapping for rest of instructions */
      {
        None
      }
    }
  }
}
