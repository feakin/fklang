use std::path::{PathBuf};
use fkl_parser::mir::LayeredArchitecture;
use crate::construct::file_resolver::FileResolver;

#[derive(Debug, Clone)]
pub struct LayeredGuardingExec<'p> {
  pub arch: &'p LayeredArchitecture,
  pub path: PathBuf,
}

impl LayeredGuardingExec<'_> {
  pub fn guarding(path: PathBuf, arch: &LayeredArchitecture) {
    let mut resolver = FileResolver::default();
    resolver.load_dir(&path);

    let exec = LayeredGuardingExec {
      arch,
      path,
    };

    println!("{:?}", exec);
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;
  use crate::exec::{LayeredGuardingExec, mir_from_file};

  #[test]
  fn test() {
    let mut base: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.push("test_data/spring");

    let mut source = base.clone();
    source.push("spring.fkl");

    let file = mir_from_file(&source);
    let arch = file.layered.unwrap();
    LayeredGuardingExec::guarding(base, &arch);
  }
}
