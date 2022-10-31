use serde::{Deserialize, Serialize};
use crate::mir::datasource::Datasource;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Environment {
  pub name: String,
  pub datasources: Vec<Datasource>,
  pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerConfig {
  pub port: u16,
}

impl Default for ServerConfig {
  fn default() -> Self {
    ServerConfig { port: 8899 }
  }
}
