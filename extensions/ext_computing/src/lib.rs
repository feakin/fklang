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
  LParen,
  RParen,
  Comma,
}

pub fn evaluate_expression(input: &str) -> Result<f64, String> {
  let tokens = tokenize(input)?;
  let mut parser = ExpressionParser::new(tokens);
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
    if command != "eval" {
      return None;
    }

    let expression = args.first()?.value.as_str();
    evaluate_expression(expression)
      .map(|value| value.to_string())
      .ok()
  }

  fn list_commands(&self) -> Vec<String> {
    vec!["eval".to_string()]
  }
}

#[no_mangle]
pub unsafe fn _fkl_create_runner() -> *mut dyn CustomRunner {
  Box::into_raw(Box::new(ComputingRunner))
}

struct ExpressionParser {
  tokens: Vec<Token>,
  cursor: usize,
}

impl ExpressionParser {
  fn new(tokens: Vec<Token>) -> Self {
    Self { tokens, cursor: 0 }
  }

  fn parse_expression(&mut self) -> Result<f64, String> {
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
      Some(Token::LParen) => {
        let value = self.parse_expression()?;
        self.expect(Token::RParen)?;
        Ok(value)
      }
      Some(Token::Ident(name)) => self.parse_function(name),
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
  use crate::evaluate_expression;

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
}
