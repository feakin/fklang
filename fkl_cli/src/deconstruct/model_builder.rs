// MIT License
//
// Copyright (c) 2021 Inherd Group
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::fs;
use std::path::{Path, PathBuf};

use crate::code_meta::{CodeFile, CodeLanguage};
use crate::deconstruct::code_construct::CodeConstruct;
use crate::deconstruct::java_construct::JavaConstruct;

pub struct ModelBuilder {}

impl ModelBuilder {
  pub fn build_models_by_dir(code_dir: PathBuf) -> Vec<CodeFile> {
    ignore::Walk::new(&code_dir)
      .filter_map(|e| CodeLanguage::filter_support_file(e))
      .map(|path| {
        ModelBuilder::model_by_file(&path)
      })
      .flatten()
      .collect()
  }

  pub fn model_by_file(path: &Path) -> Option<CodeFile> {
    let ext = path.extension().unwrap().to_str().unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    match ext {
      "java" => {
        let mut file = JavaConstruct::parse(ModelBuilder::read_content(path).as_str());
        file.path = ModelBuilder::format_path(path);
        file.file_name = file_name.to_string();
        file.pure_name = file_name.replace(".java", "");
        Some(file)
      }
      _ => None
    }
  }

  fn read_content(path: &Path) -> String {
    fs::read_to_string(path).expect("not such file")
  }

  fn format_path(path: &Path) -> String {
    format!("{}", path.display())
  }
}


#[cfg(test)]
mod tests {
  use std::env;

  use crate::deconstruct::model_builder::ModelBuilder;

  #[test]
  fn should_parse_current_dir() {
    let dir = env::current_dir().unwrap();
    let models = ModelBuilder::build_models_by_dir(dir);

    assert!(models.len() > 0);
  }
}
