use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Validation {
  Required(bool),
  Range(RangeValidation),
  Length(LengthValidation),
  Regex(RegexValidation),
  Compare(CompareValidation),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RangeValidation {
  pub min: Option<f64>,
  pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LengthValidation {
  pub min: Option<usize>,
  pub max: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RegexValidation {
  pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CompareValidation {
  pub left: String,
  pub right: String,
  pub operator: CompareOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompareOperator {
  Equal,
  NotEqual,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
}

impl Default for CompareOperator {
  fn default() -> Self {
    CompareOperator::Equal
  }
}
