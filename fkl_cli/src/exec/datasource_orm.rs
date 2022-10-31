use log::info;

use fkl_parser::mir::{Datasource, Environment};

use crate::datasource::mysql_connector::MysqlConnector;
use crate::datasource::postgres_connector::PostgresConnector;

pub(crate) async fn test_connection_runner(env: &Environment) {
  info!("test connection: {:?}", env);
  match &env.datasources[0] {
    Datasource::MySql(mysql) => {
      MysqlConnector::new(mysql.clone()).test_connection().await;
    }

    Datasource::Postgres(pgsql) => {
      PostgresConnector::from(pgsql.clone())
        .await
        .unwrap_or_else(|| panic!("cannot create connector"))
        .test_connection().await;
    }
  }
}
