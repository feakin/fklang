use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::*;

use crate::expr::ast::{BinaryOp, Expr, ExprPair, UnaryOp, Value, ValueIndex};

#[derive(Parser)]
#[grammar = "expr/grammar.pest"]
struct Calculator;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        // Precedence is defined lowest to highest
        PrattParser::new()
          .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
          .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
          .op(Op::infix(Rule::pow, Assoc::Right))
          .op(Op::postfix(Rule::fac))
          .op(Op::prefix(Rule::neg))
    };
}

pub fn parse(input: &str) {
  match Calculator::parse(Rule::program, input) {
    Ok(mut pairs) => {
      old_parser(pairs.next().unwrap().into_inner());
    },
    Err(err) => {
      println!("Error: {:?}", err);
    },
  };
}
//
// const VALUE_INDEX: usize = 0;
//
// fn parse_eval(pairs: Pairs<Rule>) -> ExprPair {
//   PRATT_PARSER
//     .map_primary(|primary| {
//       match primary.as_rule() {
//         Rule::expr => parse_eval(primary.into_inner()),
//         _ => unreachable!(),
//       }
//     })
//     .map_prefix(|op: Pair<Rule>, rhs| match op.as_rule() {
//       Rule::neg => {
//         ExprPair::new(BinaryOp::Exp, Value::UnaryOp(UnaryOp::Neg(ValueIndex(VALUE_INDEX))))
//       }
//     })
//     .map_infix(|lhs, op: Pair<Rule>, rhs| match op.as_rule() {
//       Rule::add => ExprPair::new(BinaryOp::Add, Value::UnaryOp(lhs, rhs)),
//       _ => unreachable!(),
//     })
//     .parse(pairs)
// }

// can be follow: <https://github.com/pest-parser/book/blob/master/examples/pest-calculator/src/main.rs>
fn old_parser(pairs: Pairs<Rule>) -> f64 {
  PRATT_PARSER
    .map_primary(|primary| {
      match primary.as_rule() {
        Rule::expr => old_parser(primary.into_inner()),
        Rule::int => primary.as_str().parse().unwrap(),
        Rule::num => primary.as_str().parse().unwrap(),
        Rule::function => {
          let mut inner = primary.into_inner();
          let name = inner.next().unwrap().as_str();
          let arg = inner.next().unwrap().into_inner();
          let func_name = old_parser(arg);
          execute_func(name, func_name)
        }
        _ => panic!("unimplemented, {:?}", primary.as_rule()),
      }
    })
    .map_prefix(|op, rhs: f64| match op.as_rule() {
      Rule::neg => -rhs,
      _ => panic!("unimplemented, {:?}", op.as_rule()),
    })
    .map_postfix(|lhs, op| match op.as_rule() {
      Rule::fac => (1..=lhs as u64).product::<u64>() as f64,
      _ => panic!("unimplemented, {:?}", op.as_rule()),
    })
    .map_infix(|lhs, op, rhs| match op.as_rule() {
      Rule::add => lhs + rhs,
      Rule::sub => lhs - rhs,
      Rule::mul => lhs * rhs,
      Rule::div => lhs / rhs,
      Rule::pow => lhs.powf(rhs),
      _ => panic!("unimplemented, {:?}", op.as_rule()),
    })
    .parse(pairs)
}

pub fn parse_value(pair: pest::iterators::Pair<Rule>) -> Value {
  match pair.as_rule() {
    Rule::int => Value::Const(pair.as_str().parse().unwrap()),
    Rule::variable => Value::Var(pair.as_str().to_string()),
    _ => unreachable!(),
  }
}

fn execute_func(func_name: &str, arg: f64) -> f64 {
  match func_name {
    "sin" => arg.sin(),
    "cos" => arg.cos(),
    "tan" => arg.tan(),
    "asin" => arg.asin(),
    "acos" => arg.acos(),
    "atan" => arg.atan(),
    "sinh" => arg.sinh(),
    "cosh" => arg.cosh(),
    "tanh" => arg.tanh(),
    "asinh" => arg.asinh(),
    "acosh" => arg.acosh(),
    "atanh" => arg.atanh(),
    "sqrt" => arg.sqrt(),
    "cbrt" => arg.cbrt(),
    "exp" => arg.exp(),
    "ln" => arg.ln(),
    "log2" => arg.log2(),
    "log10" => arg.log10(),
    "abs" => arg.abs(),
    "ceil" => arg.ceil(),
    "floor" => arg.floor(),
    "round" => arg.round(),
    "trunc" => arg.trunc(),
    "fract" => arg.fract(),
    _ => f64::NAN,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_expr() {
    parse("1 + 2");
    // assert_eq!(parse("1 + 2"), 3.0);
    // assert_eq!(parse("1 + 2 * 3"), 7.0);
    // assert_eq!(parse("(1 + 2) * 3"), 9.0);
    // assert_eq!(parse("1 + 2 * 3 + 4"), 11.0);
    // assert_eq!(parse("1 + 2 * (3 + 4)"), 15.0);
    // assert_eq!(parse("1 + 2 * (3 + 4) / 5"), 3.8);
    // assert_eq!(parse("1 + 2 * (3 + 4) / 5 - 6"), -2.2);
  }

  // #[test]
  // fn function_sqrt() {
  //   assert_eq!(parse("sqrt(4)"), 2.0);
  // }
}
