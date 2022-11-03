use std::time::Duration;

use log::error;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;

use fkl_mir::MySqlDatasource;

pub struct MysqlConnector {
  config: MySqlDatasource,
  pub pool: sqlx::Pool<sqlx::MySql>,
}

impl MysqlConnector {
  pub async fn new(config: MySqlDatasource) -> Option<Self> {
    let options = MySqlPoolOptions::new();

    let pool = match options
      .max_connections(5)
      .max_lifetime(Duration::from_secs(10 * 60))
      .connect(&config.url()).await {
      Ok(p) => p,
      Err(err) => {
        error!("error: {:?}", err);
        return None;
      }
    };

    Some(MysqlConnector {
      pool,
      config,
    })
  }
}

impl MysqlConnector {
  pub(crate) async fn test_connection(&self) -> bool {
    let connector: MysqlConnector = match MysqlConnector::new(self.config.clone()).await {
      None => {
        panic!("cannot create connector");
      }
      Some(connector) => connector
    };

    print!("tables: ");
    let sql = format!("SELECT * FROM information_schema.tables where table_schema = '{}'", self.config.database);
    sqlx::query(&sql)
      .map(|row: sqlx::mysql::MySqlRow| {
        let table_name: String = row.get("table_name");
        print!("{} ", table_name);
      })
      .fetch_all(&connector.pool)
      .await
      .map(|_| true)
      .unwrap_or(false)
  }
}
