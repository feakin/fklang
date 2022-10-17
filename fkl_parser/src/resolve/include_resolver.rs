use std::path::PathBuf;

pub fn include_resolver(base_path: &str, include_path: &str) -> Option<PathBuf> {
  let mut base_path = PathBuf::from(base_path);
  base_path.pop();
  base_path.push(include_path);

  base_path = base_path.canonicalize().ok()?;
  if base_path.exists() {
    Some(base_path)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_include_resolver() {
    let base_path = "/home/username/project/src/main.rs";
    let include_path = "../abc.rs";
    let result = include_resolver(base_path, include_path);
    assert_eq!(result, None);
  }

  #[test]
  fn test_include_resolver2() {
    let base_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let include_path = "fkl_parser/src/lib.rs";
    let result = include_resolver(&format!("{}", base_path.display()), include_path);
    assert!(result.is_some());
  }
}
