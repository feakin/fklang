//! refs:
//! - [https://github.com/ISibboI/evalexpr/blob/main/src/token/mod.rs]
//! - [https://github.com/likebike/fasteval/blob/master/src/compiler.rs]
//!
// The  ExprTk <https://github.com/ArashPartow/exprtk>  expression  evaluator supports  the following  fundamental
// arithmetic operations, functions and processes:
//
//  (00) Types:           Scalar, Vector, String
//
//  (01) Basic operators: +, -, *, /, %, ^
//
//  (02) Assignment:      :=, +=, -=, *=, /=, %=
//
//  (03) Equalities &
//       Inequalities:    =, ==, <>, !=, <, <=, >, >=
//
//  (04) Logic operators: and, mand, mor, nand, nor, not, or, shl, shr,
//                        xnor, xor, true, false
//
//  (05) Functions:       abs, avg, ceil, clamp, equal, erf, erfc,  exp,
//                        expm1, floor, frac,  log, log10, log1p,  log2,
//                        logn, max,  min, mul,  ncdf,  not_equal, root,
//                        round, roundn, sgn, sqrt, sum, swap, trunc
//
//  (06) Trigonometry:    acos, acosh, asin, asinh, atan, atanh,  atan2,
//                        cos,  cosh, cot,  csc, sec,  sin, sinc,  sinh,
//                        tan, tanh, hypot, rad2deg, deg2grad,  deg2rad,
//                        grad2deg
//
//  (07) Control
//       structures:      if-then-else, ternary conditional, switch-case,
//                        return-statement
//
//  (08) Loop statements: while, for, repeat-until, break, continue
//
//  (09) String
//       processing:      in, like, ilike, concatenation
//
//  (10) Optimisations:   constant-folding, simple strength reduction and
//                        dead code elimination
//
//  (11) Calculus:        numerical integration and differentiation
//
#[derive(Clone, PartialEq, Debug)]
pub enum Instruction {
  // Unary
  Not,
  Neg {
    val: Box<Instruction>,
  },
  Parenthesized,

  // Arithmetic
  Add {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
  },
  Sub {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
  },
  Mul {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
  },
  Div {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
  },
  Mod {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
  },
  Pow {
    lhs: Box<Instruction>,
    rhs: Box<Instruction>,
  },

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
  Function {
    name: String,
    args: Vec<Instruction>,
  },
}
