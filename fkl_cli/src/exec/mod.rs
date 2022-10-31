use std::fs;
use std::path::PathBuf;

use log::error;

pub use code_gen_runner::*;
pub use code_gen_runner::code_gen_by_path;
pub use code_gen_runner::layer_map::*;
pub use code_gen_runner::layer_path_builder::*;

#[allow(unused_imports)]
pub use datasource_connect::*;
pub use endpoint_runner::*;
use fkl_parser::mir::ContextMap;
use fkl_parser::parse;
pub use guarding::*;
pub use layered_guarding::*;
pub use layered_guarding::layered_guarding_exec::LayeredGuardingExec;

#[allow(unused_imports)]
pub use mock_server::*;

pub mod code_gen_runner;
pub mod layered_guarding;
pub mod endpoint_runner;
pub mod guarding;
pub mod datasource_connect;
pub mod mock_server;

pub fn mir_from_file(input_path: &PathBuf) -> ContextMap {
  let code = fs::read_to_string(input_path).unwrap();
  mir_from_str(&code)
}

pub fn mir_from_str(code: &str) -> ContextMap {
  let mir: ContextMap = parse(&code).or_else(|e| {
    error!("{}", e);
    Err(e)
  }).unwrap();

  mir
}
