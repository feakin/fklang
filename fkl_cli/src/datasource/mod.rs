use async_trait::async_trait;

pub mod mysql_connector;
pub mod postgres_connector;

#[async_trait]
pub trait DatasourceConnector {
  fn test_connection(&self) -> bool;
}
