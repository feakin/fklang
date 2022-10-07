use serde::Deserialize;
use serde::Serialize;
use crate::mir::tactic::entity::Entity;

// Cluster the entities and value objects into aggregates and define boundaries around each.
// Choose one entity to be the root of each aggregate, and allow external objects to hold
// references to the root only (references to internal members passed out for use within
// a single operation only). Define properties and invariants for the aggregate as a whole and
// give enforcement responsibility to the root or some designated framework mechanism.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Aggregate {
  pub name: String,
  pub description: String,
  pub entities: Vec<Entity>,
}

impl Aggregate {
  pub fn new(name: &str) -> Self {
    Aggregate { name: name.to_string(), description: "".to_string(), entities: vec![] }
  }
}

