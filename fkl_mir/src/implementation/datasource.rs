use serde::{Deserialize, Serialize};

/// DataSource, like mysql, postgres, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Datasource {
  MySql(MySqlDatasource),
  Postgres(PostgresDatasource),
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

  #[allow(dead_code)]
  fn url(&self) -> String {
    match self {
      Datasource::MySql(config) => MySqlDatasource::url(config),
      Datasource::Postgres(config) => format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
      ),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MySqlDatasource {
  pub host: String,
  pub port: u16,
  pub username: String,
  pub password: String,
  pub database: String,
}

impl MySqlDatasource {
  pub fn url(&self) -> String {
    format!(
      "mysql://{}:{}@{}:{}/{}",
      self.username, self.password, self.host, self.port, self.database
    )
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostgresDatasource {
  pub host: String,
  pub port: u16,
  pub username: String,
  pub password: String,
  pub database: String,
}

impl PostgresDatasource {
  pub fn url(&self) -> String {
    format!(
      "postgresql://{}:{}@{}:{}/{}",
      self.username, self.password, self.host, self.port, self.database
    )
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

  #[test]
  fn test_mysql_url_gen() {
    let datasource = Datasource::MySql(MySqlDatasource {
      host: "localhost".to_string(),
      port: 3306,
      username: "username".to_string(),
      password: "password".to_string(),
      database: "database".to_string(),
    });
    assert_eq!(datasource.url(),
      "mysql://username:password@localhost:3306/database"
    );
  }
}
