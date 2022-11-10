use serde::Deserialize;
use serde::Serialize;

use crate::tactic::block::Field;

/// Entity Object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Entity {
  pub name: String,
  pub description: String,
  pub identify: Field,
  pub fields: Vec<Field>,
}

impl Entity {
  pub fn new(name: &str) -> Self {
    Entity { name: name.to_string(), description: "".to_string(), identify: Field::default(), fields: vec![] }
  }
}

