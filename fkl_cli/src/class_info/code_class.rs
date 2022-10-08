use serde::{Deserialize, Serialize};
use crate::class_info::{CodeFunction, CodePoint};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CodeClass {
  pub name: String,
  pub path: String,
  pub module: String,
  pub package: String,
  pub functions: Vec<CodeFunction>,
  pub start: CodePoint,
  pub end: CodePoint
}
