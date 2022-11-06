#[derive(Clone, PartialEq, Debug)]
pub enum MathInstruction {
  // Arithmetic
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  // Logical
  And,
  Or,
  Xor,
  Not,
  Neg,

  // Precedence
  LBrace,
  RBrace,

  FuncSin,
  FuncCos,
}
