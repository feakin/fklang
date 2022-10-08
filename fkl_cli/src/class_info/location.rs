use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CodePoint {
  pub row: usize,
  pub column: usize
}
