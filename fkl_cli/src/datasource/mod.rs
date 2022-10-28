pub mod mysql_connector;

pub trait DatasourceConnector {
  fn test_connection(&self) -> bool;
}
