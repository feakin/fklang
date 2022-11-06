/// refs:
/// - [https://github.com/ISibboI/evalexpr/blob/main/src/token/mod.rs]
/// - [https://github.com/likebike/fasteval/blob/master/src/compiler.rs]
///
#[derive(Clone, PartialEq, Debug)]
pub enum MathInstruction {
  // Unary
  Not,
  Neg,
  Parenthesized,

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

  // Comparison
  Eq,
  Ne,
  Lt,
  Le,
  Gt,
  Ge,

  // Precedence
  LBrace,
  RBrace,

  // Functions
  FuncSin,
  FuncCos,

  // Others
  // with Builtin Types?
  Const(f64),
  Var(String),
}
