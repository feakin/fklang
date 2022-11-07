#[derive(Clone, PartialEq, Debug)]
pub struct Expr {
  pub first: Value,
  pub pairs: Vec<ExprPair>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPair(pub BinaryOp, pub Value);

impl ExprPair {
  pub fn new(op: BinaryOp, value: Value) -> Self {
    ExprPair(op, value)
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOp {
  Not,
  Neg(ValueIndex),
  Parenthesized,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ValueIndex(pub usize);

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
  Const(f64),
  Var(String),
  UnaryOp(UnaryOp),
  StdFunc(StdMathFunc),
}

#[derive(Clone, PartialEq, Debug)]
pub enum BinaryOp {
  Add {
    lhs: Value,
    rhs: Value,
  },
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
  Exp,
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

