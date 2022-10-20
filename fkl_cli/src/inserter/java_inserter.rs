use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::code_meta::CodeClass;
use crate::inserter::inserter::Inserter;
use crate::line_separator::line_separator;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct JavaInserter {}

impl Inserter for JavaInserter {
  fn insert(path: &str, clazz: &CodeClass, lines: &Vec<String>) -> Result<(), String> {
    let file_path = PathBuf::from(path);
    if !file_path.exists() {
      return Err(format!("path {} not exists", path));
    }

    let will_insert_line = clazz.end.row;

    let file = File::options()
      .read(true)
      .write(true)
      .open(&file_path).unwrap();

    let buf = BufReader::new(&file);
    let mut all_lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();

    lines.iter().enumerate().for_each(|(index, line)| {
      all_lines.insert(will_insert_line + index, line.clone());
    });

    let mut dst = File::create(&file_path).unwrap();
    dst.write(all_lines.join(&*line_separator()).as_bytes()).unwrap();

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use crate::code_meta::CodePoint;

  use super::*;

  #[test]
  fn test_insert() {
    let clazz = CodeClass {
      name: "Test".to_string(),
      path: "Test.java".to_string(),
      module: "Test".to_string(),
      package: "Test".to_string(),
      implements: vec![],
      functions: vec![],
      start: CodePoint { row: 0, column: 0 },
      end: CodePoint { row: 1, column: 0 },
    };

    let code = "public class Test {\n}";
    let path = "test.java";
    fs::write(path, code).unwrap();
    JavaInserter::insert(path, &clazz, &vec![
      "    public void demo() {".to_string(),
      "    }".to_string(),
    ]).unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "public class Test {\n    public void demo() {\n    }\n}");
  }
}
