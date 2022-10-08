#[cfg(test)]
mod tests {
  use tree_sitter::{Parser};

  #[test]
  fn test_java_ident() {
    let code = r#"
    class Test {
        int double(int x) {
            return x * 2;
        }
    }
"#;
    let mut parser = Parser::new();
    let language = tree_sitter_java::language();
    parser.set_language(language).expect("Error loading Java grammar");
    let parsed = parser.parse(code, None);
  }
}
