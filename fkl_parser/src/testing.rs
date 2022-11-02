use std::fs;
use std::path::PathBuf;
use crate::parse;

/// This function is prepare for testing only.
#[allow(dead_code)]
pub fn do_parse_test(test_name: &str) -> bool {
  let d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let mut input_path = d.clone();
  input_path.push(format!("test_data/parse/{}.fkl", test_name));

  let mut output_path = d;
  output_path.push(format!("test_data/parse/{}.txt", test_name));

  let input = fs::read_to_string(input_path).unwrap();
  let result = parse(&input).unwrap();
  let result = format!("{}", result);

  if !output_path.exists() {
    fs::write(&output_path, result).unwrap();
    panic!("Output file does not exist: {:?}! Will recreate it, please rerun!", &output_path);
  }

  let output = fs::read_to_string(output_path).unwrap();
  result == output
}

#[cfg(test)]
pub mod tests {
  use crate::testing::do_parse_test;

  #[test]
  #[cfg(not(target_os = "windows"))]
  fn parse_test() {
    assert!(do_parse_test("impl"));
  }

  #[test]
  #[cfg(not(target_os = "windows"))]
  fn layered_test() {
    assert!(do_parse_test("layered"));
  }

  #[test]
  #[cfg(not(target_os = "windows"))]
  fn aggregate_sugar_test() {
    assert!(do_parse_test("aggregate_sugar"));
  }
}
