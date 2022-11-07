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
  let namespace = EvalNamespace::new(vars, true);

  match Calculator::parse(Rule::program, input) {
    Ok(mut pairs) => {
      let expr = parse_expr(pairs.next().unwrap().into_inner());
      namespace.eval(expr)
    }
    Err(err) => {
      println!("Error: {}", err);
      f64::NAN
    }
  }
}

pub struct EvalNamespace {
  vars: BTreeMap<String, Instruction>,
}

impl EvalNamespace {
  pub fn new(vars: &BTreeMap<String, Instruction>, with_default: bool) -> EvalNamespace {
    let mut vars = vars.clone();
    if with_default {
      vars.insert("pi".to_string(), Instruction::Const(std::f64::consts::PI));
      vars.insert("e".to_string(), Instruction::Const(std::f64::consts::E));
      EvalNamespace { vars }
    } else {
      EvalNamespace { vars }
    }
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
      Instruction::Function { name, args } => {
        let args = args.into_iter().map(|arg| self.eval(arg)).collect::<Vec<f64>>();
        execute_func(&*name, args)
      }
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
        Rule::function => {
          let mut i = primary.into_inner();
          let name = i.next().unwrap().as_str().to_string();
          let args = i.map(|pair| parse_expr(pair.into_inner())).collect();
          Instruction::Function { name, args }
        }
        _ => panic!("unimplemented: {:?}", primary),
      }
    })
    .map_prefix(|op: Pair<Rule>, rhs| match op.as_rule() {
      Rule::neg => Instruction::Neg { val: Box::new(rhs) },
      _ => panic!("unimplemented: {:?}", op),
    })
    .map_infix(|lhs, op: Pair<Rule>, rhs| match op.as_rule() {
      Rule::add => Instruction::Add { lhs: Box::from(lhs), rhs: Box::from(rhs) },
      Rule::mul => Instruction::Mul { lhs: Box::from(lhs), rhs: Box::from(rhs) },
      Rule::sub => Instruction::Sub { lhs: Box::from(lhs), rhs: Box::from(rhs) },
      Rule::div => Instruction::Div { lhs: Box::from(lhs), rhs: Box::from(rhs) },
      Rule::pow => Instruction::Pow { lhs: Box::from(lhs), rhs: Box::from(rhs) },
      _ => panic!("unimplemented: {:?}", op),
    })
    .map_postfix(|lhs, op: Pair<Rule>| match op.as_rule() {
      Rule::fac => Instruction::Fac { val: Box::from(lhs) },
      _ => panic!("unimplemented: {:?}", op),
    })
    .parse(pairs)
}

fn execute_func(func_name: &str, args: Vec<f64>) -> f64 {
  match func_name {
    "sin" => args[0].sin(),
    "cos" => args[0].cos(),
    "tan" => args[0].tan(),
    "asin" => args[0].asin(),
    "acos" => args[0].acos(),
    "atan" => args[0].atan(),
    "sinh" => args[0].sinh(),
    "cosh" => args[0].cosh(),
    "tanh" => args[0].tanh(),
    "asinh" => args[0].asinh(),
    "acosh" => args[0].acosh(),
    "atanh" => args[0].atanh(),
    "sqrt" => args[0].sqrt(),
    "exp" => args[0].exp(),
    "ln" => args[0].ln(),
    "log" => args[0].log10(),
    "abs" => args[0].abs(),
    "floor" => args[0].floor(),
    "ceil" => args[0].ceil(),
    "round" => args[0].round(),
    "signum" => args[0].signum(),
    "max" => {
      let mut max = args[0];
      for arg in args {
        if arg > max {
          max = arg;
        }
      }
      max
    },
    "min" => {
      let mut min = args[0];
      for arg in args {
        if arg < min {
          min = arg;
        }
      }
      min
    },
    "pow" => args[0].powf(args[1]),
    "clamp" => args[0].max(args[1]).min(args[2]),
    _ => panic!("Function not implemented: {}", func_name),
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
      ("y".to_string(), Instruction::Const(1.0)),
    ]);
    assert_eq!(parse("1 + 2 * 3 + x + y", &map2), 10.0);
  }

  #[test]
  fn function_demo() {
    assert_eq!(parse("sqrt(4)", &Default::default()), 2.0);
    let vars = BTreeMap::from_iter(vec![("x".to_string(), Instruction::Const(2.0))]);
    assert_eq!(parse("sqrt(1 - (3 / x^2))", &vars), 0.5);
    assert_eq!(parse("sqrt(1 - (3 / 3^2))", &vars), 0.816496580927726);
  }

  #[test]
  fn sinh_demo() {
    assert_eq!(parse("sinh(0)", &Default::default()), 0.0);
    assert_eq!(parse("sinh(1)", &Default::default()), 1.1752011936438014);
    assert_eq!(parse("sinh(2)", &Default::default()), 3.6268604078470186);
    assert_eq!(parse("sinh(pi/2)", &Default::default()), 2.3012989023072947);
  }

  #[test]
  fn max_min() {
    assert_eq!(parse("max(1, 2)", &Default::default()), 2.0);
    assert_eq!(parse("min(1, 2)", &Default::default()), 1.0);
    assert_eq!(parse("max(1, 2, 3)", &Default::default()), 3.0);
    assert_eq!(parse("min(1, 2, 3)", &Default::default()), 1.0);

    let vars = BTreeMap::from_iter(vec![("x".to_string(), Instruction::Const(2.0))]);
    assert_eq!(parse("1.2 + max(1, 2, 3, x)", &vars), 4.2);
  }

  #[test]
  fn const_e() {
    let vars2 = BTreeMap::from_iter(vec![("x".to_string(), Instruction::Const(2.0))]);
    assert_eq!(parse("sin(2.34e-3 * x)", &vars2), 0.004679982916146709);

    let mut symbol_table = exprtk_rs::SymbolTable::new();
    symbol_table.add_variable("x", 2.0).unwrap().unwrap();
    let mut expr = exprtk_rs::Expression::new("sin(2.34e-3 * x)", symbol_table).unwrap();

    assert_eq!(expr.value(), 0.004679982916146709);
  }

  #[test]
  fn clamp() {
    let vars2 = BTreeMap::from_iter(vec![
      ("x".to_string(), Instruction::Const(2.0)),
      ("y".to_string(), Instruction::Const(2.0)),
      ("pi".to_string(), Instruction::Const(std::f64::consts::PI)),
    ]);
    assert_eq!(parse("clamp(-1, sin(2 * pi * x) + cos(y / 2 * pi), +1)", &vars2), -1.0);

    let mut symbol_table = exprtk_rs::SymbolTable::new();
    symbol_table.add_variable("x", 2.0).unwrap().unwrap();
    symbol_table.add_variable("y", 2.0).unwrap().unwrap();
    symbol_table.add_variable("pi", std::f64::consts::PI).unwrap().unwrap();

    let mut expr = exprtk_rs::Expression::new("clamp(-1, sin(2 * pi * x) + cos(y / 2 * pi), +1)", symbol_table).unwrap();

    assert_eq!(expr.value(), -1.0);
  }
}
