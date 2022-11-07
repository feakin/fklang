use std::collections::BTreeMap;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest::pratt_parser::*;

use crate::expr::token::Instruction;

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

pub fn parse(input: &str, vars: &BTreeMap<String, Instruction>) -> f64 {
  let namespace = EvalNamespace::new(vars);
  match Calculator::parse(Rule::program, input) {
    Ok(mut pairs) => {
      let expr = parse_expr(pairs.next().unwrap().into_inner());
      namespace.eval(expr)
    }
    Err(_err) => {
      f64::NAN
    }
  }
}

pub struct EvalNamespace<'b> {
  vars: &'b BTreeMap<String, Instruction>,
}

impl<'b> EvalNamespace<'b> {
  pub fn new(vars: &BTreeMap<String, Instruction>) -> EvalNamespace {
    EvalNamespace { vars }
  }

  fn eval(&self, ins: Instruction) -> f64 {
    match ins {
      Instruction::Const(v) => v,
      Instruction::Add { lhs, rhs } => self.eval(*lhs) + self.eval(*rhs),
      Instruction::Sub { lhs, rhs } => self.eval(*lhs) - self.eval(*rhs),
      Instruction::Mul { lhs, rhs } => self.eval(*lhs) * self.eval(*rhs),
      Instruction::Div { lhs, rhs } => self.eval(*lhs) / self.eval(*rhs),
      Instruction::Pow { lhs, rhs } => self.eval(*lhs).powf(self.eval(*rhs)),
      Instruction::Neg { val } => -self.eval(*val),
      Instruction::Var(value) => {
        let var = self.vars.get(&value).unwrap();
        self.eval(var.clone())
      }
      // Instruction::Fac(a) => (1..=eval(*a) as u64).product::<u64>() as f64,
      _ => panic!("Not implemented: {:?}", ins),
    }
  }
}

fn parse_expr(pairs: Pairs<Rule>) -> Instruction {
  PRATT_PARSER
    .map_primary(|primary| {
      match primary.as_rule() {
        Rule::expr => parse_expr(primary.into_inner()),
        Rule::num => {
          let num = primary.as_str().parse::<f64>().unwrap();
          Instruction::Const(num)
        }
        Rule::ident => {
          Instruction::Var(primary.as_str().to_string())
        }
        _ => panic!("unimplemented: {:?}", primary),
      }
    })
    .map_prefix(|op: Pair<Rule>, rhs| match op.as_rule() {
      Rule::neg => Instruction::Neg { val: Box::new(rhs) },
      _ => panic!("unimplemented: {:?}", op),
    })
    .map_infix(|lhs, op: Pair<Rule>, rhs| match op.as_rule() {
      Rule::add => Instruction::Add {
        lhs: Box::from(lhs),
        rhs: Box::from(rhs),
      },
      Rule::mul => Instruction::Mul {
        lhs: Box::from(lhs),
        rhs: Box::from(rhs),
      },
      _ => panic!("unimplemented: {:?}", op),
    })
    .parse(pairs)
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
    assert_eq!(parse("1 + 2", &Default::default()), 3.0);
    assert_eq!(parse("1 + 2 * 3", &Default::default()), 7.0);
    let map: BTreeMap<String, Instruction> = BTreeMap::from_iter(vec![("y".to_string(), Instruction::Const(1.0))]);
    assert_eq!(parse("1 + 2 * 3 + y", &map), 8.0);

    let map2: BTreeMap<String, Instruction> = BTreeMap::from_iter(vec![
      ("x".to_string(), Instruction::Const(2.0)),
      ("y".to_string(), Instruction::Const(1.0))
    ]);
    assert_eq!(parse("1 + 2 * 3 + x + y", &map2), 10.0);
  }

  #[test]
  fn function_sqrt() {
    assert_eq!(parse("sqrt(4)", &Default::default()), 2.0);
  }
}
