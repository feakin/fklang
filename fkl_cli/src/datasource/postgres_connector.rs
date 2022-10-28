use std::time::Duration;

use log::{error, info};
use sqlx::postgres::PgPoolOptions;

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

    match options
      .max_connections(5)
      .max_lifetime(Duration::from_secs(10 * 60))
      .connect(&self.config.url()).await {
      Ok(p) => {
        info!("p: {:?}", p);
        true
      }
      Err(err) => {
        error!("error: {:?}", err);
        false
      }
    }
  }
}
