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
