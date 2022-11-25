use std::collections::HashMap;
use std::fmt::Display;
use serde::Deserialize;
use serde::Serialize;

use crate::{BoundedContext, ConnectionDirection, ContextRelation, Entity, LayeredArchitecture, SourceSets, Step, Struct};
use crate::environment::Environment;
use crate::implementation::Implementation;

///
/// Identify each model in play on the project and define its bounded context. This includes
/// the implicit models of non-object-oriented subsystems. Name each bounded context, and make
/// the names part of the ubiquitous language.
///
/// Describe the points of contact between the models, outlining explicit translation for
/// any communication, highlighting any sharing, isolation mechanisms, and levels of influence.
///
/// Map the existing terrain. Take up transformations later.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ContextMap {
  pub name: String,
  pub state: ContextState,
  pub contexts: Vec<BoundedContext>,
  pub relations: Vec<ContextRelation>,
  pub implementations: Vec<Implementation>,
  pub layered: Option<LayeredArchitecture>,
  pub source_sets: Option<SourceSets>,
  pub envs: Vec<Environment>,
  pub structs: HashMap<String, Struct>,
  // todo: create a symbol table for the context map
}

impl ContextMap {
  pub fn get_entity(&self, entity_name: &str) -> Option<Entity> {
    return self.contexts.iter().find_map(|bc| {
      bc.aggregates.iter().find_map(|aggregate| {
        aggregate.entities.iter().find_map(|entity| {
          Self::filter_by_name(entity_name, entity)
        })
      })
    })
  }

  fn filter_by_name(entity_name: &str, entity: &Entity) -> Option<Entity> {
    if entity.name.to_lowercase() == entity_name.to_lowercase() {
      Some(entity.clone())
    } else {
      None
    }
  }

  pub fn get_struct(&self, struct_name: &str) -> Option<Struct> {
    self.structs.get(struct_name).map(|s| s.clone())
  }
}

#[allow(dead_code)]
impl ContextMap {
  // todo: from symbol table?
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
    for relation in &self.relations {
      let rel = match relation.connection_type {
        ConnectionDirection::Undirected => "-",
        ConnectionDirection::PositiveDirected => "->",
        ConnectionDirection::NegativeDirected => "<-",
        ConnectionDirection::BiDirected => "<->",
      };
      writeln!(f, "  Relation({} {} {}) ", relation.source, rel, relation.target)?;
    }

    for context in &self.contexts {
      writeln!(f, "  BoundedContext({})", context.name)?;
      for aggregate in &context.aggregates {
        writeln!(f, "    Aggregate({})", aggregate.name)?;
        for entity in &aggregate.entities {
          writeln!(f, "      Entity({})", entity.name)?;
          entity.fields.iter().for_each(|field| {
            writeln!(f, "        Field({})", field.name).unwrap();
          });
        }
      }
    }

    for imp in &self.implementations {
      match imp {
        Implementation::PublishHttpApi(api) => {
          writeln!(f, "    PublishHttpApi({})", api.name)?;
          writeln!(f, "      {:?} Path({})", api.endpoint.method, api.endpoint.path)?;

          if let Some(request) = &api.endpoint.request {
            writeln!(f, "      Request: {}", request.name)?;
          }

          if let Some(response) = &api.endpoint.response {
            writeln!(f, "      Response: {}", response.name)?;
          }

          api.flow.iter().for_each(|flow| {
            writeln!(f, "      Flow").unwrap();
            flow.steps.iter().for_each(|step| {
              match step {
                Step::MethodCall(call) => {
                  writeln!(f, "        MethodCall({})", call.name).unwrap();
                }
                Step::Message(msg) => {
                  writeln!(f, "        Message({})", msg.from).unwrap();
                }
                Step::RpcCall(_) => {}
              }
            });
          });
        }
        Implementation::PublishEvent => {}
        Implementation::PublishMessage => {}
      }
    }

    self.layered.as_ref().map(|layered| {
      writeln!(f, "  LayeredArchitecture({})", layered.name).unwrap();
      for layer in &layered.layers {
        writeln!(f, "    Layer {} (\"{}\")", layer.name, layer.package).unwrap();
      }
    });

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::ContextMap;
  use crate::{Aggregate, BoundedContext, Entity};

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
              identify: Default::default(),
              fields: vec![],
            }],
          }
        ],
      }],
      relations: vec![],
      implementations: vec![],
      layered: None,
      source_sets: None,
      envs: vec![],
      structs: Default::default(),
    };
    let output = format!("{}", context_map);
    assert_eq!(output, r#"ContextMap(Ticket)
  BoundedContext(TicketContext)
    Aggregate(TicketAggregate)
      Entity(TicketEntity)
"#);
  }
}
