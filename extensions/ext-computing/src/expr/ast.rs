#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
  Int(i64),
  // Convert(Box<Expr>, Type),

}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOp {
  Not,
  Neg,
  Parenthesized
}

#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,
  BitAnd,
  BitOr,
  BitXor,
  BitShl,
  BitShr,
  And,
  Or,
  Eq,
  Ne,
  Lt,
  Le,
  Gt,
  Ge,
}

#[derive(Clone, PartialEq, Debug)]
pub enum StdMathFunc {
  Abs,
  Acos,
  Acosh,
  Asin,
  Asinh,
  Atan,
  Atanh,
  Atan2,
  Cbrt,
  Ceil,
  Cos,
  Cosh,
  Exp,
  Exp2,
  ExpM1,
  Floor,
  Fract,
  Hypot,
  Ln,
  Ln1p,
  Log,
  Log10,
  Log2,
  Logb,
  Max,
  Min,
  Powf,
  Round,
  Sin,
  Sinh,
  Sqrt,
  Tan,
  Tanh,
  Trunc,
}

