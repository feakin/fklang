pub mod mysql_connector;
use async_trait::async_trait;

#[async_trait]
pub trait DatasourceConnector {
  fn test_connection(&self) -> bool;
}
