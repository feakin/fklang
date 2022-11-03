use serde::{Deserialize, Serialize};

use crate::default_config;
use crate::VariableDefinition;
use crate::datasource::Datasource;

/// Global environment configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Environment {
  pub name: String,
  pub datasources: Vec<Datasource>,
  pub server: ServerConfig,
  pub customs: Vec<CustomEnv>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerConfig {
  pub port: u16,
}

impl Default for ServerConfig {
  fn default() -> Self {
    ServerConfig { port: default_config::SERVER_PORT }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct CustomEnv {
  pub name: String,
  pub attrs: Vec<VariableDefinition>,
}

