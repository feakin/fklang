use std::fs;
use std::path::PathBuf;

use log::error;

pub use code_gen_exec::code_gen_by_path;
pub use code_gen_exec::layer_map::*;
pub use code_gen_exec::layer_path_builder::*;

pub use layered_guarding_exec::LayeredGuardingExec;

use fkl_parser::mir::ContextMap;
use fkl_parser::parse;

pub mod code_gen_exec;
pub mod layered_guarding_exec;

pub fn mir_from_file(input_path: &PathBuf) -> ContextMap {
  let code = fs::read_to_string(input_path).unwrap();
  let mir: ContextMap = parse(&code).or_else(|e| {
    error!("{}", e);
    Err(e)
  }).unwrap();

  mir
}
