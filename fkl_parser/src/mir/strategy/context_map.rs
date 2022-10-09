use std::fmt::Display;
use serde::Deserialize;
use serde::Serialize;

use crate::mir::{BoundedContext, ContextRelation, LayeredArchitecture};
use crate::mir::implementation::Implementation;

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

  pub implementations: Vec<Implementation>,
  // todo: add rest in the future
  // some entities no in map
  // pub rest_entities: Vec<Entity>,
  // some value objects no in map
  // pub rest_value_objects: Vec<ValueObject>,
  pub layered: Option<LayeredArchitecture>,
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

impl Display for ContextMap {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "ContextMap({})", self.name)?;
    for context in &self.contexts {
      writeln!(f, "  BoundedContext({})", context.name)?;
      for aggregate in &context.aggregates {
        writeln!(f, "    Aggregate({})", aggregate.name)?;
        for entity in &aggregate.entities {
          writeln!(f, "      Entity({})", entity.name)?;
        }
      }
    }

    writeln!(f, "")?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::ContextMap;
  use crate::mir::{Aggregate, BoundedContext, Entity};

  #[test]
  fn display_context_map() {
    let context_map = ContextMap {
      name: "Ticket".to_string(),
      state: Default::default(),
      contexts: vec![BoundedContext {
        name: "TicketContext".to_string(),
        aggregates: vec![
          Aggregate {
            name: "TicketAggregate".to_string(),
            description: "".to_string(),
            entities: vec![Entity {
              name: "TicketEntity".to_string(),
              description: "".to_string(),
              is_aggregate_root: false,
              identify: Default::default(),
              fields: vec![]
            }]
          }
        ]
      }],
      relations: vec![],
      implementations: vec![],
      layered: None
    };
    let output = format!("{}", context_map);
    assert_eq!(output, r#"ContextMap(Ticket)
  BoundedContext(TicketContext)
    Aggregate(TicketAggregate)
      Entity(TicketEntity)

"#);
  }
}
