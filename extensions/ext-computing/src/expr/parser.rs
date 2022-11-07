use pest::iterators::Pairs;
use pest::Parser;
use pest::pratt_parser::*;

#[derive(Parser)]
#[grammar = "expr/grammar.pest"]
struct Calculator;

pub fn parse(input: &str) -> f64 {
  let parse_result = Calculator::parse(Rule::calculation, input);
  match parse_result {
    Ok(r) => eval(r),
    Err(_) => f64::NAN,
  }
}

fn eval(pairs: Pairs<Rule>) -> f64 {
  PrattParser::new()
    .map_primary(|pair| match pair.as_rule() {
      Rule::calculation => eval(pair.into_inner()),
      Rule::expr => eval(pair.into_inner()),
      Rule::num => pair.as_str().trim().parse::<f64>().unwrap(),
      _ => panic!("unimplemented, {:?}", pair.as_rule()),
    })
    .map_infix(|lhs, op, rhs| match op.as_rule() {
      Rule::add => lhs + rhs,
      _ => panic!("unimplemented, {:?}", op.as_rule()),
    })
    .parse(pairs);

  0.0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn it_works() {
    assert_eq!(parse("1 + 2"), 3.0);
    assert_eq!(parse("1 + 2 * 3"), 7.0);
    assert_eq!(parse("(1 + 2) * 3"), 9.0);
    assert_eq!(parse("1 + 2 * 3 + 4"), 11.0);
    assert_eq!(parse("1 + 2 * (3 + 4)"), 15.0);
    assert_eq!(parse("1 + 2 * (3 + 4) / 5"), 3.0);
    assert_eq!(parse("1 + 2 * (3 + 4) / 5 - 6"), -3.0);
  }
}
