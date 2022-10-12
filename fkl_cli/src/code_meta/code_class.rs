use serde::{Deserialize, Serialize};
use crate::code_meta::{CodeFunction, CodePoint, Location};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeClass {
  pub name: String,
  pub path: String,
  pub module: String,
  pub package: String,
  pub implements: Vec<String>,
  pub functions: Vec<CodeFunction>,
  pub start: CodePoint,
  pub end: CodePoint,
}

impl CodeClass {
  pub fn is_contain_method(&self, method_name: &str) -> bool {
    self.functions.iter().any(|func| func.name == method_name)
  }
}

impl Location for CodeClass {
  fn set_start(&mut self, row: usize, column: usize) {
    self.start.row = row;
    self.start.column = column;
  }

  fn set_end(&mut self, row: usize, column: usize) {
    self.end.row = row;
    self.end.column = column;
  }
}
