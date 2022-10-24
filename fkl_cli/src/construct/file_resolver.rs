use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// inspired by [Solang](https://github.com/hyperledger-labs/solang)
pub struct FileResolver {
  cached_paths: HashMap<PathBuf, usize>,
  files: HashMap<PathBuf, Arc<str>>,
}

#[derive(Clone, Debug)]
pub struct ResolvedFile {
  path: PathBuf,
  import_paths: Vec<(Option<OsString>, PathBuf)>,
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

    self.files.insert(path.to_path_buf(), Arc::from(content));
    self.cached_paths.insert(path.to_path_buf(), pos);

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
