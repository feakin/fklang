use serde::{Deserialize, Serialize};
use crate::class_info::{CodeClass, CodeFunction};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CodeFile {
  pub file_name: String,
  pub path: String,
  pub package: String,
  pub imports: Vec<String>,
  pub classes: Vec<CodeClass>,
  pub functions: Vec<CodeFunction>,
}
