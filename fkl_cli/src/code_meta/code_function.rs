use serde::{Deserialize, Serialize};
use crate::code_meta::{CodePoint, Location};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeFunction {
  pub name: String,
  // todo: add support
  pub return_type: String,
  // todo: add support
  pub variable: Vec<String>,
  pub start: CodePoint,
  pub end: CodePoint,
}


impl Location for CodeFunction {
  fn set_start(&mut self, row: usize, column: usize) {
    self.start.row = row;
    self.start.column = column;
  }

  fn set_end(&mut self, row: usize, column: usize) {
    self.end.row = row;
    self.end.column = column;
  }
}
