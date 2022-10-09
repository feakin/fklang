use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LayeredArchitecture {
  pub name: String,
  pub description: String,
  pub dependencies: Vec<Dependency>,
  pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Dependency {
  pub source: String,
  pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Layer {
  pub name: String,
  pub package: String,
}
