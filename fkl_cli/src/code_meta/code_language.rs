use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CodeLanguage {
  Java,
  Kotlin,
  Rust,
  TypeScript,
}

impl Default for CodeLanguage {
  fn default() -> Self {
    CodeLanguage::Java
  }
}
