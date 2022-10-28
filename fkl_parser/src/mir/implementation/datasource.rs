use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Datasource {
  MySql(MySqlDatasource),
  Postgres(PostgresDatasource),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MySqlDatasource {
  pub host: String,
  pub port: u16,
  pub username: String,
  pub password: String,
  pub database: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostgresDatasource {
  pub host: String,
  pub port: u16,
  pub username: String,
  pub password: String,
  pub database: String,
}

impl Datasource {
  pub fn from(url: &str) -> Result<Datasource, String> {
    let url = url::Url::parse(url).map_err(|e| e.to_string())?;
    let scheme = url.scheme();
    let host = url.host_str().ok_or("host is required")?.to_string();
    let port = url.port().ok_or("port is required")?;
    let username = url.username().to_string();
    let password = url.password().unwrap_or("").to_string();
    let database = url.path().trim_start_matches('/').to_string();

    match scheme {
      "mysql" => Ok(Datasource::MySql(MySqlDatasource {
        host,
        port,
        username,
        password,
        database,
      })),
      "postgresql" => Ok(Datasource::Postgres(PostgresDatasource {
        host,
        port,
        username,
        password,
        database,
      })),
      _ => Err(format!("unsupported scheme: {}", scheme)),
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_database_url() {
    let url = "mysql://username:password@localhost:3306/database";
    let datasource = Datasource::from(url).unwrap();
    assert_eq!(
      datasource,
      Datasource::MySql(MySqlDatasource {
        host: "localhost".to_string(),
        port: 3306,
        username: "username".to_string(),
        password: "password".to_string(),
        database: "database".to_string(),
      })
    );
  }

  #[test]
  fn test_parse_normal_postgres() {
    let url = "postgresql://localhost:5432/yourdb";
    let datasource = Datasource::from(url).unwrap();

    assert_eq!(
      datasource,
      Datasource::Postgres(PostgresDatasource {
        host: "localhost".to_string(),
        port: 5432,
        username: "".to_string(),
        password: "".to_string(),
        database: "yourdb".to_string(),
      })
    );
  }
}
