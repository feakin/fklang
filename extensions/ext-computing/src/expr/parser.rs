use pest::iterators::Pairs;
use pest::Parser;
use pest::pratt_parser::*;

#[derive(Parser)]
#[grammar = "expr/grammar.pest"]
struct Calculator;

pub fn parse(input: &str) -> f64 {
  let parse_result = Calculator::parse(Rule::program, input);
  match parse_result {
    Ok(r) => eval(r),
    Err(_) => f64::NAN,
  }
}

fn eval(pairs: Pairs<Rule>) -> f64 {
  let pratt =
    PrattParser::new()
      .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
      .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
      .op(Op::infix(Rule::pow, Assoc::Right))
      .op(Op::postfix(Rule::fac))
      .op(Op::prefix(Rule::neg));

  parse_expr(pairs, &pratt)
}

fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> f64 {
  pratt
    .map_primary(|primary| match primary.as_rule() {
      Rule::int => primary.as_str().parse().unwrap(),
      Rule::expr => parse_expr(primary.into_inner(), pratt), // from "(" ~ expr ~ ")"
      Rule::num => primary.as_str().parse().unwrap(),
      _ => panic!("unimplemented, {:?}", primary.as_rule()),
    })
    .map_prefix(|op, rhs| match op.as_rule() {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn it_works() {
    assert_eq!(parse("1 + 2"), 3.0);
    // assert_eq!(parse("1 + 2 * 3"), 7.0);
    // assert_eq!(parse("(1 + 2) * 3"), 9.0);
    // assert_eq!(parse("1 + 2 * 3 + 4"), 11.0);
    // assert_eq!(parse("1 + 2 * (3 + 4)"), 15.0);
    // assert_eq!(parse("1 + 2 * (3 + 4) / 5"), 3.0);
    // assert_eq!(parse("1 + 2 * (3 + 4) / 5 - 6"), -3.0);
  }
}
