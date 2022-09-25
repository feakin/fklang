use serde::Deserialize;
use serde::Serialize;

use crate::mir::{BoundedContext, ContextRelation};

//
// Identify each model in play on the project and define its bounded context. This includes
// the implicit models of non-object-oriented subsystems. Name each bounded context, and make
// the names part of the ubiquitous language.
//
// Describe the points of contact between the models, outlining explicit translation for
// any communication, highlighting any sharing, isolation mechanisms, and levels of influence.
//
// Map the existing terrain. Take up transformations later.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContextMap {
  pub name: String,
  pub state: ContextState,
  pub contexts: Vec<BoundedContext>,
  pub relations: Vec<ContextRelation>,

  // todo: add rest in the future
  // some entities no in map
  // pub rest_entities: Vec<Entity>,
  // some value objects no in map
  // pub rest_value_objects: Vec<ValueObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContextState {
  AsIs,
  ToBe,
}

impl Default for ContextState {
  fn default() -> Self {
    ContextState::ToBe
  }
}
