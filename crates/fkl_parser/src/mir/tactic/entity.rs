use serde::Deserialize;
use serde::Serialize;

use crate::mir::tactic::block::Field;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Entity {
  pub name: String,
  pub description: String,
  pub is_aggregate_root: bool,
  pub identify: Field,
  pub fields: Vec<Field>,
}

impl Entity {
  pub fn new(name: &str) -> Self {
    Entity { name: name.to_string(), description: "".to_string(), is_aggregate_root: false, identify: Field::default(), fields: vec![] }
  }
}

