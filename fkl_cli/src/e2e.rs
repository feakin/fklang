#[cfg(test)]
mod tests {
  use std::path::PathBuf;
  use crate::code_gen_exec;

  #[test]
  fn test_java_package_to_path() {
    let mut d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/spring");

    let base_path = d.clone();

    let mut input_path = d.clone();
    input_path.push(format!("spring.fkl"));

    code_gen_exec::code_gen_exec(Some(&input_path), Some(&"HelloGot".to_string()), &base_path);
  }
}
