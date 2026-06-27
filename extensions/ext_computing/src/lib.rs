use std::collections::HashMap;

use async_trait::async_trait;
use fkl_ext_api::custom_runner::{Argument, CustomRunner};
use fkl_mir::{ContextMap, CustomEnv};

pub mod function_type;

#[derive(Debug, Clone, PartialEq)]
enum Token {
  Number(f64),
  Ident(String),
  Plus,
  Minus,
  Star,
  Slash,
  Bang,
  AndAnd,
  OrOr,
  EqualEqual,
  NotEqual,
  Less,
  LessEqual,
  Greater,
  GreaterEqual,
  LParen,
  RParen,
  Comma,
}

pub fn evaluate_expression(input: &str) -> Result<f64, String> {
  evaluate_expression_with_vars(input, &HashMap::new())
}

pub fn evaluate_expression_with_vars(
  input: &str,
  variables: &HashMap<String, f64>,
) -> Result<f64, String> {
  let tokens = tokenize(input)?;
  let mut parser = ExpressionParser::new(tokens, variables);
  let value = parser.parse_expression()?;
  if parser.is_done() {
    Ok(value)
  } else {
    Err("unexpected token after expression".to_string())
  }
}

pub struct ComputingRunner;

#[async_trait]
impl CustomRunner for ComputingRunner {
  fn name(&self) -> &str {
    "computing"
  }

  async fn execute(&self, _context: &ContextMap, _env: &CustomEnv) {}

  async fn send_command(&self, command: &str, args: &[Argument]) -> Option<String> {
    match command {
      "eval" => {
        let expression = args.first()?.value.as_str();
        evaluate_expression(expression)
          .map(|value| value.to_string())
          .ok()
      }
      "filter" => {
        let values = args.first()?.value.as_str();
        let predicate = args.get(1)?.value.as_str();
        filter_numbers(values, predicate).ok().map(|values| {
          values
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(",")
        })
      }
      "repl" => {
        let input = args.first()?.value.as_str();
        run_repl_lines(input).ok().map(|outputs| outputs.join("\n"))
      }
      _ => None,
    }
  }

  fn list_commands(&self) -> Vec<String> {
    vec!["eval".to_string(), "filter".to_string(), "repl".to_string()]
  }
}

#[no_mangle]
pub unsafe fn _fkl_create_runner() -> *mut dyn CustomRunner {
  Box::into_raw(Box::new(ComputingRunner))
}

pub fn filter_numbers(input: &str, predicate: &str) -> Result<Vec<f64>, String> {
  let mut filtered = Vec::new();

  for value in parse_number_list(input)? {
    let mut variables = HashMap::new();
    variables.insert("x".to_string(), value);
    if is_truthy(evaluate_expression_with_vars(predicate, &variables)?) {
      filtered.push(value);
    }
  }

  Ok(filtered)
}

pub fn run_repl_lines(input: &str) -> Result<Vec<String>, String> {
  let mut outputs = Vec::new();

  for line in input.lines() {
    let expression = line.trim();
    if expression.is_empty() {
      continue;
    }
    if expression.eq_ignore_ascii_case("exit") || expression.eq_ignore_ascii_case("quit") {
      break;
    }

    outputs.push(evaluate_expression(expression)?.to_string());
  }

  Ok(outputs)
}

fn parse_number_list(input: &str) -> Result<Vec<f64>, String> {
  input
    .split(',')
    .map(str::trim)
    .filter(|value| !value.is_empty())
    .map(|value| {
      value
        .parse::<f64>()
        .map_err(|_| format!("invalid filter value: {}", value))
    })
    .collect()
}

struct ExpressionParser<'a> {
  tokens: Vec<Token>,
  cursor: usize,
  variables: &'a HashMap<String, f64>,
}

impl<'a> ExpressionParser<'a> {
  fn new(tokens: Vec<Token>, variables: &'a HashMap<String, f64>) -> Self {
    Self {
      tokens,
      cursor: 0,
      variables,
    }
  }

  fn parse_expression(&mut self) -> Result<f64, String> {
    self.parse_logical_or()
  }

  fn parse_logical_or(&mut self) -> Result<f64, String> {
    let mut value = self.parse_logical_and()?;

    while let Some(Token::OrOr) = self.peek() {
      self.advance();
      let rhs = self.parse_logical_and()?;
      value = bool_to_number(is_truthy(value) || is_truthy(rhs));
    }

    Ok(value)
  }

  fn parse_logical_and(&mut self) -> Result<f64, String> {
    let mut value = self.parse_equality()?;

    while let Some(Token::AndAnd) = self.peek() {
      self.advance();
      let rhs = self.parse_equality()?;
      value = bool_to_number(is_truthy(value) && is_truthy(rhs));
    }

    Ok(value)
  }

  fn parse_equality(&mut self) -> Result<f64, String> {
    let mut value = self.parse_comparison()?;

    while let Some(token) = self.peek() {
      match token {
        Token::EqualEqual => {
          self.advance();
          value = bool_to_number(value == self.parse_comparison()?);
        }
        Token::NotEqual => {
          self.advance();
          value = bool_to_number(value != self.parse_comparison()?);
        }
        _ => break,
      }
    }

    Ok(value)
  }

  fn parse_comparison(&mut self) -> Result<f64, String> {
    let mut value = self.parse_additive()?;

    while let Some(token) = self.peek() {
      match token {
        Token::Less => {
          self.advance();
          value = bool_to_number(value < self.parse_additive()?);
        }
        Token::LessEqual => {
          self.advance();
          value = bool_to_number(value <= self.parse_additive()?);
        }
        Token::Greater => {
          self.advance();
          value = bool_to_number(value > self.parse_additive()?);
        }
        Token::GreaterEqual => {
          self.advance();
          value = bool_to_number(value >= self.parse_additive()?);
        }
        _ => break,
      }
    }

    Ok(value)
  }

  fn parse_additive(&mut self) -> Result<f64, String> {
    let mut value = self.parse_term()?;

    while let Some(token) = self.peek() {
      match token {
        Token::Plus => {
          self.advance();
          value += self.parse_term()?;
        }
        Token::Minus => {
          self.advance();
          value -= self.parse_term()?;
        }
        _ => break,
      }
    }

    Ok(value)
  }

  fn parse_term(&mut self) -> Result<f64, String> {
    let mut value = self.parse_factor()?;

    while let Some(token) = self.peek() {
      match token {
        Token::Star => {
          self.advance();
          value *= self.parse_factor()?;
        }
        Token::Slash => {
          self.advance();
          value /= self.parse_factor()?;
        }
        _ => break,
      }
    }

    Ok(value)
  }

  fn parse_factor(&mut self) -> Result<f64, String> {
    match self.advance() {
      Some(Token::Number(value)) => Ok(value),
      Some(Token::Minus) => Ok(-self.parse_factor()?),
      Some(Token::Bang) => Ok(bool_to_number(!is_truthy(self.parse_factor()?))),
      Some(Token::LParen) => {
        let value = self.parse_expression()?;
        self.expect(Token::RParen)?;
        Ok(value)
      }
      Some(Token::Ident(name)) => match name.as_str() {
        "true" => Ok(1.0),
        "false" => Ok(0.0),
        _ if self.peek() == Some(&Token::LParen) => self.parse_function(name),
        _ if self.variables.contains_key(&name) => Ok(self.variables[&name]),
        _ => Err(format!("unknown identifier: {}", name)),
      },
      Some(token) => Err(format!("unexpected token: {:?}", token)),
      None => Err("unexpected end of expression".to_string()),
    }
  }

  fn parse_function(&mut self, name: String) -> Result<f64, String> {
    self.expect(Token::LParen)?;

    let mut args = Vec::new();
    if self.peek() == Some(&Token::RParen) {
      self.advance();
    } else {
      loop {
        args.push(self.parse_expression()?);
        match self.peek() {
          Some(Token::Comma) => {
            self.advance();
          }
          Some(Token::RParen) => {
            self.advance();
            break;
          }
          other => return Err(format!("expected comma or right paren, got {:?}", other)),
        }
      }
    }

    match name.as_str() {
      "sum" => Ok(args.iter().sum()),
      _ => Err(format!("unknown function: {}", name)),
    }
  }

  fn expect(&mut self, expected: Token) -> Result<(), String> {
    let actual = self.advance();
    if actual == Some(expected.clone()) {
      Ok(())
    } else {
      Err(format!("expected {:?}, got {:?}", expected, actual))
    }
  }

  fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.cursor)
  }

  fn advance(&mut self) -> Option<Token> {
    let token = self.tokens.get(self.cursor).cloned();
    self.cursor += usize::from(token.is_some());
    token
  }

  fn is_done(&self) -> bool {
    self.cursor == self.tokens.len()
  }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().peekable();

  while let Some(ch) = chars.peek().copied() {
    match ch {
      ' ' | '\t' | '\n' | '\r' => {
        chars.next();
      }
      '0'..='9' | '.' => {
        tokens.push(Token::Number(read_number(&mut chars)?));
      }
      'a'..='z' | 'A'..='Z' | '_' => {
        tokens.push(Token::Ident(read_ident(&mut chars)));
      }
      '+' => {
        chars.next();
        tokens.push(Token::Plus);
      }
      '-' => {
        chars.next();
        tokens.push(Token::Minus);
      }
      '*' => {
        chars.next();
        tokens.push(Token::Star);
      }
      '/' => {
        chars.next();
        tokens.push(Token::Slash);
      }
      '!' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::NotEqual);
        } else {
          tokens.push(Token::Bang);
        }
      }
      '&' => {
        chars.next();
        if chars.peek() == Some(&'&') {
          chars.next();
          tokens.push(Token::AndAnd);
        } else {
          return Err("expected '&' after '&'".to_string());
        }
      }
      '|' => {
        chars.next();
        if chars.peek() == Some(&'|') {
          chars.next();
          tokens.push(Token::OrOr);
        } else {
          return Err("expected '|' after '|'".to_string());
        }
      }
      '=' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::EqualEqual);
        } else {
          return Err("expected '=' after '='".to_string());
        }
      }
      '<' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::LessEqual);
        } else {
          tokens.push(Token::Less);
        }
      }
      '>' => {
        chars.next();
        if chars.peek() == Some(&'=') {
          chars.next();
          tokens.push(Token::GreaterEqual);
        } else {
          tokens.push(Token::Greater);
        }
      }
      '(' => {
        chars.next();
        tokens.push(Token::LParen);
      }
      ')' => {
        chars.next();
        tokens.push(Token::RParen);
      }
      ',' => {
        chars.next();
        tokens.push(Token::Comma);
      }
      _ => return Err(format!("unexpected character: {}", ch)),
    }
  }

  Ok(tokens)
}

fn bool_to_number(value: bool) -> f64 {
  if value {
    1.0
  } else {
    0.0
  }
}

fn is_truthy(value: f64) -> bool {
  value != 0.0
}

fn read_number<I>(chars: &mut std::iter::Peekable<I>) -> Result<f64, String>
where
  I: Iterator<Item = char>,
{
  let mut number = String::new();
  while let Some(ch) = chars.peek().copied() {
    if ch.is_ascii_digit() || ch == '.' {
      number.push(ch);
      chars.next();
    } else {
      break;
    }
  }

  number
    .parse::<f64>()
    .map_err(|_| format!("invalid number: {}", number))
}

fn read_ident<I>(chars: &mut std::iter::Peekable<I>) -> String
where
  I: Iterator<Item = char>,
{
  let mut ident = String::new();
  while let Some(ch) = chars.peek().copied() {
    if ch.is_ascii_alphanumeric() || ch == '_' {
      ident.push(ch);
      chars.next();
    } else {
      break;
    }
  }
  ident
}

#[cfg(test)]
mod tests {
  use fkl_ext_api::custom_runner::CustomRunner;

  use crate::{evaluate_expression, filter_numbers, run_repl_lines, ComputingRunner};

  #[test]
  fn evaluates_integer_addition_and_precedence() {
    assert_eq!(evaluate_expression("1 + 2 * 3").unwrap(), 7.0);
  }

  #[test]
  fn evaluates_division_with_parentheses() {
    assert_eq!(evaluate_expression("(8 / 2) + 3").unwrap(), 7.0);
  }

  #[test]
  fn evaluates_sum_function() {
    assert_eq!(evaluate_expression("sum(1, 2, 3)").unwrap(), 6.0);
  }

  #[test]
  fn evaluates_logic_expressions() {
    assert_eq!(evaluate_expression("true && false").unwrap(), 0.0);
    assert_eq!(evaluate_expression("true || false").unwrap(), 1.0);
    assert_eq!(evaluate_expression("!false").unwrap(), 1.0);
  }

  #[test]
  fn evaluates_comparison_expressions() {
    assert_eq!(evaluate_expression("1 + 2 * 3 > 6 && 4 <= 4").unwrap(), 1.0);
    assert_eq!(
      evaluate_expression("sum(1, 2, 3) == 7 || 3 != 3").unwrap(),
      0.0
    );
  }

  #[test]
  fn filters_numbers_with_predicate_expression() {
    assert_eq!(
      filter_numbers("1, 2, 3, 4", "x >= 3").unwrap(),
      vec![3.0, 4.0]
    );
    assert_eq!(
      filter_numbers("1, 2, 3, 4", "x > 1 && x < 4").unwrap(),
      vec![2.0, 3.0]
    );
  }

  #[test]
  fn lists_filter_command() {
    assert!(ComputingRunner
      .list_commands()
      .contains(&"filter".to_string()));
  }

  #[test]
  fn evaluates_repl_lines_until_exit() {
    assert_eq!(
      run_repl_lines("1 + 2\ntrue && false\nexit\n3 + 4").unwrap(),
      vec!["3".to_string(), "0".to_string()]
    );
  }

  #[test]
  fn lists_repl_command() {
    assert!(ComputingRunner
      .list_commands()
      .contains(&"repl".to_string()));
  }
}
