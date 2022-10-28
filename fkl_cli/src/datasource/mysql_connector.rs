use std::time::Duration;

use log::error;
use sqlx::mysql::MySqlPoolOptions;

use fkl_parser::mir::MySqlDatasource;

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

impl MysqlConnector {
  pub(crate) async fn test_connection(&self) -> bool {
    let options = MySqlPoolOptions::new();

    match options
      .max_connections(5)
      .max_lifetime(Duration::from_secs(10 * 60))
      .connect(&self.config.url()).await {
      Ok(_) => {
        true
      }
      Err(err) => {
        error!("error: {:?}", err);
        false
      }
    }
  }
}
