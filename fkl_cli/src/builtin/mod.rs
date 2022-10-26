use std::path::PathBuf;
use std::process;

use log::error;

use fkl_parser::mir::LayeredArchitecture;

use crate::exec::LayeredGuardingExec;

pub mod endpoint_runner;

pub fn guarding_runner(root: PathBuf, layered: &LayeredArchitecture) {
  let errors = LayeredGuardingExec::guarding(root, layered);

  if errors.len() > 0 {
    for error in errors {
      error!("error layered: {}", error);
    }

    process::exit(0x0100);
  }
}
