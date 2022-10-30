use log::error;
use sqlx::sqlite::SqlitePoolOptions;
use fkl_parser::mir::SqliteDatasource;

pub struct SqliteConnector {
  config: SqliteDatasource,
}

impl SqliteConnector {
  pub fn new(config: SqliteDatasource) -> Self {
    SqliteConnector {
      config
    }
  }
}

impl SqliteConnector {
  pub(crate) async fn test_connection(&self) -> bool {
    let options = SqlitePoolOptions::new();

    match options
      .max_connections(5)
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
