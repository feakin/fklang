use std::path::PathBuf;
use std::process;

use log::{error, info};

use fkl_parser::mir::{Datasource, Environment, LayeredArchitecture};

use crate::datasource::mysql_connector::MysqlConnector;
use crate::datasource::postgres_connector::PostgresConnector;
use crate::datasource::sqlite_connector::SqliteConnector;
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

pub(crate) async fn test_connection_runner(env: &Environment) {
  info!("test connection: {:?}", env);
  match &env.datasources[0] {
    Datasource::MySql(mysql) => {
      MysqlConnector::new(mysql.clone()).test_connection().await;
    }

    Datasource::Postgres(pgsql) => {
      PostgresConnector::new(pgsql.clone()).test_connection().await;
    }
    Datasource::Sqlite(sql) => {
      SqliteConnector::new(sql.clone()).test_connection().await;
    }
  }
}
