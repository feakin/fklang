use serde::{Deserialize, Serialize};
use crate::class_info::CodePoint;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CodeFunction {
  pub name: String,
  pub vars: Vec<String>,
  pub start: CodePoint,
  pub end: CodePoint,
}
