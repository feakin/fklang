use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LayeredArchitecture {
  pub name: String,
  pub description: String,
  pub relations: Vec<LayerRelation>,
  pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LayerRelation {
  // todo: add identify vs string with Enum ?
  pub source: String,
  pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Layer {
  pub name: String,
  pub package: String,
}
