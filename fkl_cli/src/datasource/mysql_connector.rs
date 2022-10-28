use sqlx::mysql::MySqlPoolOptions;
use fkl_parser::mir::MySqlDatasource;
use crate::datasource::DatasourceConnector;

pub struct MysqlConnector {
  config: MySqlDatasource,
}

impl MysqlConnector {
  pub fn new(config: MySqlDatasource) -> Self {
    MysqlConnector {
      config
    }
  }
}

// impl DatasourceConnector for MysqlConnector {
//   fn test_connection(&self) -> bool {
//     let pool = MySqlPoolOptions::new()
//       .max_connections(5)
//       .connect("postgres://postgres:password@localhost/test").await?;
//
//
//   }
// }
