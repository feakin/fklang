use fkl_parser::mir::{Datasource, Environment};
use log::info;
use crate::datasource::mysql_connector::MysqlConnector;
use crate::datasource::postgres_connector::PostgresConnector;

pub(crate) async fn test_connection_runner(env: &Environment) {
  info!("test connection: {:?}", env);
  match &env.datasources[0] {
    Datasource::MySql(mysql) => {
      MysqlConnector::new(mysql.clone()).test_connection().await;
    }

    Datasource::Postgres(pgsql) => {
      PostgresConnector::new(pgsql.clone()).test_connection().await;
    }
  }
}
