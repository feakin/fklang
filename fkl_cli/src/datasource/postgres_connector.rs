use std::time::Duration;

use log::error;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

use fkl_parser::mir::PostgresDatasource;
use crate::builtin::builtin_type::BuiltinType;

pub struct PostgresConnector {
  pool: sqlx::Pool<sqlx::Postgres>,
  config: PostgresDatasource,
}

impl PostgresConnector {
  pub async fn from(config: PostgresDatasource) -> Option<Self> {
    let options = PgPoolOptions::new();

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

    Some(PostgresConnector {
      pool,
      config,
    })
  }
}

impl PostgresConnector {
  pub(crate) async fn test_connection(&self) -> bool {
    let connector: PostgresConnector = match PostgresConnector::from(self.config.clone()).await {
      None => {
        panic!("cannot create connector");
      }
      Some(connector) => connector
    };

    print!("tables: ");
    let sql = format!("SELECT * FROM {}.information_schema.tables where table_schema = 'public'", self.config.database);
    sqlx::query(&sql)
      .map(|row: sqlx::postgres::PgRow| {
        let table_name: String = row.get("table_name");
        print!("{} ", table_name);
      })
      .fetch_all(&connector.pool)
      .await
      .map(|_| true)
      .unwrap_or(false)
  }

  pub(crate) async fn get_tables(&self) -> Vec<String> {
    let sql = format!("SELECT * FROM {}.information_schema.tables where table_schema = 'public'", self.config.database);
    sqlx::query(&sql)
      .map(|row: sqlx::postgres::PgRow| {
        let table_name: String = row.get("table_name");
        table_name
      })
      .fetch_all(&self.pool)
      .await
      .unwrap_or(vec![])
  }

  // select column_name, data_type from information_schema.columns  where table_name = 'employee'
  // return Vec<TableInfo>
  pub async fn get_table_info(&self, table_name: &str) -> Vec<TableInfo> {
    let sql = format!("select column_name, data_type from {}.information_schema.columns  where table_name = '{}'", self.config.database, table_name);
    let mut table_info = Vec::new();

    let rows = sqlx::query(&sql)
      .fetch_all(&self.pool)
      .await
      .unwrap();

    for row in rows {
      let column_name: String = row.get("column_name");
      let data_type: String = row.get("data_type");
      table_info.push(TableInfo {
        column_name,
        data_type,
      });
    }

    table_info
  }

  /// postgres data_type to builtin type
  /// refs: link https://www.postgresql.org/docs/current/datatype.html
  /// ```markdown
  /// | Name | Aliases | Description |
  /// | --- | --- | --- |
  /// | `bigint` | `int8` | signed eight-byte integer |
  /// | `bigserial` | `serial8` | autoincrementing eight-byte integer |
  /// | `bit [ (_`n`_) ]` |   | fixed-length bit string |
  /// | `bit varying [ (_`n`_) ]` | `varbit [ (_`n`_) ]` | variable-length bit string |
  /// | `boolean` | `bool` | logical Boolean (true/false) |
  /// | `box` |   | rectangular box on a plane |
  /// | `bytea` |   | binary data (“byte array”) |
  /// | `character [ (_`n`_) ]` | `char [ (_`n`_) ]` | fixed-length character string |
  /// | `character varying [ (_`n`_) ]` | `varchar [ (_`n`_) ]` | variable-length character string |
  /// | `cidr` |   | IPv4 or IPv6 network address |
  /// | `circle` |   | circle on a plane |
  /// | `date` |   | calendar date (year, month, day) |
  /// | `double precision` | `float8` | double precision floating-point number (8 bytes) |
  /// | `inet` |   | IPv4 or IPv6 host address |
  /// | `integer` | `int`, `int4` | signed four-byte integer |
  /// | `interval [ _`fields`_ ] [ (_`p`_) ]` |   | time span |
  /// | `json` |   | textual JSON data |
  /// | `jsonb` |   | binary JSON data, decomposed |
  /// | `line` |   | infinite line on a plane |
  /// | `lseg` |   | line segment on a plane |
  /// | `macaddr` |   | MAC (Media Access Control) address |
  /// | `macaddr8` |   | MAC (Media Access Control) address (EUI-64 format) |
  /// | `money` |   | currency amount |
  /// | `numeric [ (_`p`_, _`s`_) ]` | `decimal [ (_`p`_, _`s`_) ]` | exact numeric of selectable precision |
  /// | `path` |   | geometric path on a plane |
  /// | `pg_lsn` |   | PostgreSQL Log Sequence Number |
  /// | `pg_snapshot` |   | user-level transaction ID snapshot |
  /// | `point` |   | geometric point on a plane |
  /// | `polygon` |   | closed geometric path on a plane |
  /// | `real` | `float4` | single precision floating-point number (4 bytes) |
  /// | `smallint` | `int2` | signed two-byte integer |
  /// | `smallserial` | `serial2` | autoincrementing two-byte integer |
  /// | `serial` | `serial4` | autoincrementing four-byte integer |
  /// | `text` |   | variable-length character string |
  /// | `time [ (_`p`_) ] [ without time zone ]` |   | time of day (no time zone) |
  /// | `time [ (_`p`_) ] with time zone` | `timetz` | time of day, including time zone |
  /// | `timestamp [ (_`p`_) ] [ without time zone ]` |   | date and time (no time zone) |
  /// | `timestamp [ (_`p`_) ] with time zone` | `timestamptz` | date and time, including time zone |
  /// | `tsquery` |   | text search query |
  /// | `tsvector` |   | text search document |
  /// | `txid_snapshot` |   | user-level transaction ID snapshot (deprecated; see `pg_snapshot`) |
  /// | `uuid` |   | universally unique identifier |
  /// | `xml` |   | XML data |
  /// ```
  pub fn data_type_to_builtin_type(data_type: &str) -> BuiltinType {
    match data_type {
      "bigint" => BuiltinType::Integer,
      "bigserial" => BuiltinType::Integer,
      "bit" => BuiltinType::String,
      "bit varying" => BuiltinType::String,
      "boolean" => BuiltinType::Boolean,
      "box" => BuiltinType::String,
      "bytea" => BuiltinType::String,
      "character" => BuiltinType::String,
      "character varying" => BuiltinType::String,
      "cidr" => BuiltinType::String,
      "circle" => BuiltinType::String,
      "date" => BuiltinType::Date,
      "double precision" => BuiltinType::Float,
      "inet" => BuiltinType::String,
      "integer" => BuiltinType::Integer,
      "interval" => BuiltinType::String,
      "json" => BuiltinType::String,
      "jsonb" => BuiltinType::String,
      "line" => BuiltinType::String,
      "lseg" => BuiltinType::String,
      "macaddr" => BuiltinType::String,
      "macaddr8" => BuiltinType::String,
      "money" => BuiltinType::String,
      "numeric" => BuiltinType::String,
      "path" => BuiltinType::String,
      "pg_lsn" => BuiltinType::String,
      "pg_snapshot" => BuiltinType::String,
      "point" => BuiltinType::String,
      "polygon" => BuiltinType::String,
      "real" => BuiltinType::Float,
      "smallint" => BuiltinType::Integer,
      "smallserial" => BuiltinType::Integer,
      "serial" => BuiltinType::Integer,
      "text" => BuiltinType::String,
      "time" => BuiltinType::DateTime,
      "time with time zone" => BuiltinType::DateTime,
      "timestamp" => BuiltinType::Timestamp,
      "timestamp with time zone" => BuiltinType::String,
      "tsquery" => BuiltinType::String,
      "tsvector" => BuiltinType::String,
      "txid_snapshot" => BuiltinType::String,
      "uuid" => BuiltinType::Special("uuid".to_string()),
      "xml" => BuiltinType::String,
      _ => BuiltinType::String
    }
  }
}

#[derive(Debug, Clone)]
pub struct TableInfo {
  pub column_name: String,
  pub data_type: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  #[ignore]
  async fn test_connection() {
    let config = PostgresDatasource {
      host: "localhost".to_string(),
      port: 5432,
      username: "".to_string(),
      password: "".to_string(),
      database: "test".to_string(),
    };

    let connector = PostgresConnector::from(config).await.unwrap();
    assert!(connector.test_connection().await);
  }

  #[tokio::test]
  #[ignore]
  async fn test_get_tables() {
    let config = PostgresDatasource {
      host: "localhost".to_string(),
      port: 5432,
      username: "".to_string(),
      password: "".to_string(),
      database: "test".to_string(),
    };

    let connector = PostgresConnector::from(config).await.unwrap();
    let tables = connector.get_tables().await;
    assert_eq!(tables.len(), 2);
  }

  #[tokio::test]
  #[ignore]
  async fn test_get_table_info() {
    let config = PostgresDatasource {
      host: "localhost".to_string(),
      port: 5432,
      username: "".to_string(),
      password: "".to_string(),
      database: "test".to_string(),
    };

    let connector = PostgresConnector::from(config).await.unwrap();
    let table_info = connector.get_table_info("employee").await;
    assert_eq!(table_info.len(), 6);
    println!("{:?}", table_info);
  }

  #[test]
  fn test_string_date_type() {
    let data_type = "date";
    let builtin_type = PostgresConnector::data_type_to_builtin_type(data_type);
    assert_eq!(builtin_type, BuiltinType::Date);
  }
}
