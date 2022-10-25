use std::ffi::OsStr;
use serde::{Deserialize, Serialize};

/// CodeLanguage is a enum type to represent the language of code file.
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

impl CodeLanguage {
  /// return supported extensions name
  pub fn supported() -> Vec<String> {
    vec!["java".to_string(), "kt".to_string(), "rs".to_string(), "ts".to_string()]
  }

  pub(crate) fn is_support(ext: &OsStr) -> bool {
    let ext = ext.to_str().unwrap();
    CodeLanguage::supported().contains(&ext.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_supported() {
    let supported = CodeLanguage::supported();
    assert_eq!(supported.len(), 4);
    assert_eq!(supported[0], "java");
    assert_eq!(supported[1], "kt");
    assert_eq!(supported[2], "rs");
    assert_eq!(supported[3], "ts");
  }

  #[test]
  fn test_is_support() {
    assert!(CodeLanguage::is_support("java".as_ref()));
    assert!(CodeLanguage::is_support("kt".as_ref()));
    assert!(CodeLanguage::is_support("rs".as_ref()));
    assert!(CodeLanguage::is_support("ts".as_ref()));
    assert!(!CodeLanguage::is_support("js".as_ref()));
  }
}

