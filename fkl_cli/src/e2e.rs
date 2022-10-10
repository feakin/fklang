#[cfg(test)]
mod tests {
  use std::path::PathBuf;
  use crate::code_gen_exec;

  #[test]
  fn test_java_package_to_path() {
    let d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let base_path = d.clone();

    let mut input_path = d.clone();
    input_path.push(format!("test_data/spring/spring.fkl"));

    code_gen_exec::code_gen_exec(Some(&input_path), Some(&"HelloGot".to_string()), &base_path);
  }
}
