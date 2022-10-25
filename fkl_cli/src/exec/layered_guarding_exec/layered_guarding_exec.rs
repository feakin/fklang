use std::path::PathBuf;
use fkl_parser::mir::LayeredArchitecture;
use crate::construct::file_resolver::{FileResolver, ResolvedFile};
use crate::exec::layered_guarding_exec::package_guarding::PackageGuarding;

#[derive(Debug, Clone)]
pub struct LayeredGuardingExec<'p> {
  pub package_guarding: &'p PackageGuarding,
  pub path: PathBuf,
}

impl LayeredGuardingExec<'_> {
  pub fn guarding(path: PathBuf, arch: &LayeredArchitecture) -> Vec<String> {
    let mut resolver = FileResolver::default();
    resolver.load_dir(&path);

    let guarding = PackageGuarding::from_arch(arch);

    let exec = LayeredGuardingExec {
      package_guarding: &guarding,
      path,
    };

    resolver.files.iter()
      .map(|(_path, file)| {
        exec.guarding_file(file)
      })
      .flatten()
      .collect()
  }

  fn guarding_file(&self, file: &ResolvedFile) -> Vec<String> {
    if let Some(code) = &file.meta {
      return self.package_guarding.verify(code);
    }

    vec![]
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;
  use crate::exec::mir_from_file;
  use crate::exec::layered_guarding_exec::layered_guarding_exec::LayeredGuardingExec;

  #[test]
  fn test() {
    let mut base: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.push("test_data/spring");

    let mut source = base.clone();
    source.push("spring.fkl");

    let file = mir_from_file(&source);
    let arch = file.layered.unwrap();
    let errors = LayeredGuardingExec::guarding(base, &arch);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0], "package com.feakin.demo.domain imported com.feakin.demo.rest")
  }
}
