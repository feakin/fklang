use serde::{Deserialize, Serialize};
use crate::class_info::{CodePoint, Location};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CodeFunction {
  pub name: String,
  pub vars: Vec<String>,
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
