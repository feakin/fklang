use std::{fs, process};
use std::path::PathBuf;

use futures::executor::block_on;
use log::{error, info};

pub use code_gen_exec::*;
pub use code_gen_exec::code_gen_by_path;
pub use code_gen_exec::layer_map::*;
pub use code_gen_exec::layer_path_builder::*;
pub use datasource_connect::*;
pub use endpoint_runner::*;
use fkl_parser::mir::{ContextMap, Datasource, Environment, LayeredArchitecture};
use fkl_parser::parse;
pub use guarding::*;
pub use layered_guarding::*;
pub use layered_guarding::layered_guarding_exec::LayeredGuardingExec;
pub use mock_server::*;

use crate::datasource::mysql_connector::MysqlConnector;
use crate::datasource::postgres_connector::PostgresConnector;
use crate::mock::stub_server::feakin_rocket;

pub mod code_gen_exec;
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
