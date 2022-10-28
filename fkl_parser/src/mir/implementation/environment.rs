use serde::{Deserialize, Serialize};
use crate::mir::datasource::Datasource;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Environment {
  pub name: String,
  pub datasources: Vec<Datasource>,
}
