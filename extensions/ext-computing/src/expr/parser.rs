use pest::iterators::Pairs;
use pest::Parser;
use pest::pratt_parser::*;

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

pub fn parse(input: &str) -> f64 {
  match Calculator::parse(Rule::program, input) {
    Ok(mut pairs) => parse_expr(pairs.next().unwrap().into_inner()),
    Err(_) => f64::NAN,
  }
}

// can be follow: <https://github.com/pest-parser/book/blob/master/examples/pest-calculator/src/main.rs>
fn parse_expr(pairs: Pairs<Rule>) -> f64 {
  PRATT_PARSER
    .map_primary(|primary| match primary.as_rule() {
      Rule::expr => parse_expr(primary.into_inner()),
      Rule::int => primary.as_str().parse().unwrap(),
      Rule::num => primary.as_str().parse().unwrap(),
      _ => panic!("unimplemented, {:?}", primary.as_rule()),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(parse("1 + 2"), 3.0);
    assert_eq!(parse("1 + 2 * 3"), 7.0);
    assert_eq!(parse("(1 + 2) * 3"), 9.0);
    assert_eq!(parse("1 + 2 * 3 + 4"), 11.0);
    assert_eq!(parse("1 + 2 * (3 + 4)"), 15.0);
    assert_eq!(parse("1 + 2 * (3 + 4) / 5"), 3.8);
    assert_eq!(parse("1 + 2 * (3 + 4) / 5 - 6"), -2.2);
  }
}
