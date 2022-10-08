use serde::{Deserialize, Serialize};

pub trait Location {
  fn set_start(&mut self, row: usize, column: usize);
  fn set_end(&mut self, row: usize, column: usize);
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CodePoint {
  pub row: usize,
  pub column: usize
}
