use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf};
use std::sync::Arc;

use crate::code_meta::{CodeFile, CodeLanguage};
use crate::construct::model_builder::ModelBuilder;

// inspired by [Solang](https://github.com/hyperledger-labs/solang)
pub struct FileResolver {
  cached_paths: HashMap<PathBuf, usize>,
  files: HashMap<PathBuf, ResolvedFile>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ResolvedFile {
  content: Arc<str>,
  imports: Vec<String>,
  exports: Vec<String>,
  language: CodeLanguage,
  meta: Option<CodeFile>,
  path: PathBuf,
}

impl Default for FileResolver {
  fn default() -> Self {
    FileResolver {
      cached_paths: Default::default(),
      files: Default::default(),
    }
  }
}

impl FileResolver {
  pub fn load_dir(&mut self, path: &PathBuf) {
    for entry in walkdir::WalkDir::new(path) {
      let entry = entry.unwrap();
      if !entry.file_type().is_file() {
        continue;
      }

      let path = entry.path().to_path_buf();
      if let None = path.extension() {
        continue;
      }

      // java only
      if let Some(ext) = path.extension() {
        if !CodeLanguage::is_support(ext) {
          continue;
        }
      }

      self.load_file(&path).unwrap();
    }
  }

  fn load_file(&mut self, path: &PathBuf) -> Result<(), String> {
    if self.cached_paths.get(path).is_some() {
      return Ok(());
    }

    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(err) => return Err(format!("{}: {}", path.display(), err))
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
      Ok(_) => (),
      Err(err) => return Err(format!("{}: {}", path.display(), err))
    }

    let pos = self.files.len();
    self.cached_paths.insert(path.to_path_buf(), pos);

    let meta = ModelBuilder::model_by_file(path);
    let mut imports: Vec<String> = vec![];
    let mut exports: Vec<String> = vec![];

    if let Some(file) = &meta {
      imports = file.imports.clone();
      exports.push(format!("{}.{}", file.package, file.pure_name));
    }

    let resolved_file = ResolvedFile {
      content: Arc::from(content),
      imports,
      exports,
      language: CodeLanguage::Java,
      meta,
      path: path.to_path_buf(),
    };

    self.files.insert(path.to_path_buf(), resolved_file);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_file() {
    let mut resolver = FileResolver::default();

    let mut d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/spring/src/main/java/com/feakin/demo/DemoApplication.java");

    resolver.load_file(&d).unwrap();

    assert_eq!(resolver.files.len(), 1);
    assert_eq!(resolver.cached_paths.len(), 1);

    let file = resolver.files.get(&d).unwrap();
    assert_eq!(file.language, CodeLanguage::Java);
    assert_eq!(file.meta.is_some(), true);
    assert_eq!(file.imports.len(), 2);
    assert_eq!(file.imports[0], "org.springframework.boot.SpringApplication");
    assert_eq!(file.meta.as_ref().unwrap().classes[0].name, "DemoApplication");

    assert_eq!(file.exports.len(), 1);
    assert_eq!(file.exports[0], "com.feakin.demo.DemoApplication");
  }

  #[test]
  fn test_load_dir() {
    let mut resolver = FileResolver::default();

    let mut d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/spring/src/main/java/com/feakin/demo");

    resolver.load_dir(&d);

    assert_eq!(resolver.files.len(), 2);
    assert_eq!(resolver.cached_paths.len(), 2);
  }
}
