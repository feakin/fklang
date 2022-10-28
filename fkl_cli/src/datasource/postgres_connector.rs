use std::time::Duration;

use log::{error, info};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

use fkl_parser::mir::PostgresDatasource;

pub struct PostgresConnector {
  config: PostgresDatasource,
}

impl PostgresConnector {
  pub fn new(config: PostgresDatasource) -> Self {
    PostgresConnector {
      config
    }
  }
}

impl PostgresConnector {
  pub(crate) async fn test_connection(&self) -> bool {
    let options = PgPoolOptions::new();

    let pool = match options
      .max_connections(5)
      .max_lifetime(Duration::from_secs(10 * 60))
      .connect(&self.config.url()).await {
      Ok(p) => p,
      Err(err) => {
        error!("error: {:?}", err);
        return false;
      }
    };

    let sql = format!("SELECT * FROM {}.information_schema.tables where table_schema = 'public'", self.config.database);
    sqlx::query(&sql)
      .map(|row: sqlx::postgres::PgRow| {
        let table_name: String = row.get("table_name");
        info!("table: {}", table_name);
      })
      .fetch_all(&pool)
      .await
      .map(|_| true)
      .unwrap_or(false)
  }
}
