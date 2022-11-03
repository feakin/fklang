use std::fs;
use std::path::PathBuf;

use log::error;

pub use code_gen::*;
pub use code_gen::code_gen_by_path;
pub use code_gen::layer_map::*;
pub use code_gen::layer_path_builder::*;
#[allow(unused_imports)]
pub use datasource_orm::*;
use fkl_mir::ContextMap;
use fkl_parser::parse;
pub use http_request::*;
pub use layered_guarding::*;
pub use layered_guarding::layered_guarding_exec::LayeredGuardingExec;
#[allow(unused_imports)]
pub use mock_server::*;
#[allow(unused_imports)]
pub use custom_function::*;
// pub use message::*;

pub mod code_gen;
pub mod layered_guarding;
pub mod http_request;
pub mod datasource_orm;
pub mod mock_server;
pub mod message;
pub mod custom_function;

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
