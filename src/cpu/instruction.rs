pub enum Instruction {
  ADD(ArithmeticTarget),
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
