use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::code_meta::{CodeFile, CodeLanguage};

// inspired by [Solang](https://github.com/hyperledger-labs/solang)
pub struct FileResolver {
  cached_paths: HashMap<PathBuf, usize>,
  files: HashMap<PathBuf, ResolvedFile>,
}

#[derive(Clone, Debug)]
pub struct ResolvedFile {
  content: Arc<str>,
  import_paths: Vec<(String, PathBuf)>,
  language: CodeLanguage,
  meta: CodeFile,
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
  fn load_file(&mut self, path: &Path) -> Result<(), String> {
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

    let resolved_file = ResolvedFile {
      content: Arc::from(content),
      import_paths: Default::default(),
      language: CodeLanguage::Java,
      meta: Default::default(),
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
  }
}
