use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::class_info::CodeClass;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct JavaInserter {}

impl JavaInserter {
  fn insert(&self, path: &str, clazz: &CodeClass, code: Vec<String>) -> Result<(), String> {
    let file_path = PathBuf::from(path);
    if !file_path.exists() {
      return Err(format!("path {} not exists", path));
    }

    let will_insert_line = clazz.end.row - 1;

    let file = File::options()
      .read(true)
      .write(true)
      .open(&file_path).unwrap();

    let buf = BufReader::new(&file);
    buf.lines().enumerate().for_each(|(index, line)| {
      if index == will_insert_line {
        let mut writer = LineWriter::new(&file);
        code.iter().for_each(|line| {
          writer.write(line.as_bytes()).unwrap();
        });
      }
    });

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::class_info::CodePoint;

  use super::*;

  #[test]
  #[ignore]
  fn test_insert() {
    let inserter = JavaInserter {};
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
    inserter.insert(path, &clazz, vec![
      "    public void demo() {\n".to_string(),
      "    }".to_string(),
    ]).unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "public class Test {public class Test {");
  }
}
