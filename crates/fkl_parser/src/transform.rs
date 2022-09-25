use std::collections::HashMap;
use indexmap::IndexMap;

use crate::{ContextMap, mir, ParseError};
use crate::mir::{BoundedContext, ConnectionDirection, ContextRelationType, Entity, Field, ValueObject};
use crate::mir::tactic::aggregate::Aggregate;
use crate::parser::ast::{AggregateDecl, BoundedContextDecl, EntityDecl, FklDeclaration, RelationDirection, VariableDefinition};
use crate::parser::parse as ast_parse;

#[derive(Debug, PartialEq, Eq)]
pub struct MirTransform {
  pub context_map_name: String,
  pub contexts: IndexMap<String, BoundedContext>,
  // pub contexts: HashMap<String, BoundedContext>,
  pub relations: Vec<mir::ContextRelation>,

  pub aggregates: HashMap<String, Aggregate>,
  pub entities: IndexMap<String, Entity>,
  pub value_objects: IndexMap<String, ValueObject>,
}

impl MirTransform {
  pub fn mir(str: &str) -> Result<ContextMap, ParseError> {
    let mut transform = MirTransform {
      context_map_name: "".to_string(),
      contexts: Default::default(),
      aggregates: Default::default(),
      relations: vec![],
      entities: Default::default(),
      value_objects: Default::default(),
    };

    match ast_parse(str) {
      Ok(decls) => {
        transform.lower_decls(decls);
      }
      Err(e) => return Err(e),
    };

    let contexts = transform.update_aggregates();

    Ok(ContextMap {
      name: transform.context_map_name,
      state: Default::default(),
      contexts,
      relations: transform.relations,
    })
  }

  fn update_aggregates(&self) -> Vec<BoundedContext> {
    let mut contexts = vec![];

    self.contexts.values().for_each(|origin| {
      let mut context: BoundedContext = BoundedContext::new(&origin.name);

      for aggregate in origin.aggregates.clone() {
        if let Some(agg) = self.aggregates.get(&aggregate.name) {
          context.aggregates.push(agg.clone());
        } else {
          context.aggregates.push(aggregate.clone())
        }
      }

      contexts.push(context);
    });

    contexts
  }

  fn lower_decls(&mut self, decls: Vec<FklDeclaration>) {
    decls.iter().for_each(|declaration| {
      match declaration {
        FklDeclaration::None => {}
        FklDeclaration::ContextMap(context_map) => {
          self.context_map_name = context_map.name.name.clone();

          context_map.contexts.iter().for_each(|context_decl| {
            let bounded_context = Self::transform_bounded_context(&context_decl);

            self.contexts.insert(bounded_context.name.clone(), bounded_context);
          });

          context_map.relations.iter().for_each(|relation| {
            let rel = mir::ContextRelation {
              source: relation.source.clone(),
              target: relation.target.clone(),
              connection_type: transform_connection(&relation.direction),
              source_type: ContextRelationType::list(&relation.source_types),
              target_type: ContextRelationType::list(&relation.target_types),
            };
            self.relations.push(rel);
          });
        }
        FklDeclaration::BoundedContext(bc) => {
          let bounded_context = Self::transform_bounded_context(&bc);
          self.contexts.insert(bounded_context.name.clone(), bounded_context);
        }
        FklDeclaration::Domain(_) => {}
        FklDeclaration::Aggregate(decl) => {
          let aggregate = self.transform_aggregate(&decl);
          self.aggregates.insert(aggregate.name.clone(), aggregate.clone());
        }
        FklDeclaration::DomainService(_) => {}
        FklDeclaration::ApplicationService(_) => {}
        FklDeclaration::Entity(entity_decl) => {
          let entity = self.transform_entity(&entity_decl);
          self.entities.insert(entity.name.clone(), entity.clone());
        }
        FklDeclaration::ValueObject(_) => {}
        FklDeclaration::Component(_) => {}
      }
    });
  }

  fn transform_bounded_context(context_decl: &&BoundedContextDecl) -> BoundedContext {
    let mut bounded_context = mir::BoundedContext::new(&context_decl.name);
    context_decl.used_domain_objects.iter().for_each(|domain_object| {
      bounded_context.aggregates.push(Aggregate::new(&domain_object.name.clone()));
    });

    bounded_context
  }

  fn transform_aggregate(&mut self, decl: &AggregateDecl) -> mir::Aggregate {
    let mut aggregate = mir::Aggregate::new(&decl.name);
    decl.used_domain_objects.iter().for_each(|domain_object| {
      let entity = Entity::new(&domain_object.name);
      aggregate.entities.push(entity);
    });

    aggregate
  }

  fn transform_entity(&mut self, decl: &EntityDecl) -> mir::Entity {
    let mut entity = mir::Entity::new(&decl.name);
    entity.description = decl.inline_doc.clone();
    entity.is_aggregate_root = decl.is_aggregate_root;

    entity.fields = decl.fields.iter().map(|field| Self::transform_field(field)).collect();

    entity
  }

  fn transform_field(field: &VariableDefinition) -> Field {
    Field {
      initializer: field.initializer.clone(),
      name: field.name.clone(),
      type_type: field.type_type.clone(),
    }
  }
}

fn transform_connection(rd: &RelationDirection) -> ConnectionDirection {
  match rd {
    RelationDirection::Undirected => ConnectionDirection::Undirected,
    RelationDirection::PositiveDirected => ConnectionDirection::PositiveDirected,
    RelationDirection::NegativeDirected => ConnectionDirection::NegativeDirected,
    RelationDirection::BiDirected => ConnectionDirection::BiDirected,
  }
}

#[cfg(test)]
mod tests {
  use crate::mir::{Aggregate, BoundedContext, ContextRelation, ContextRelationType, Entity};
  use crate::mir::ConnectionDirection::PositiveDirected;
  use crate::mir::tactic::block::Field;
  use crate::transform::MirTransform;

  #[test]
  fn basic_mir() {
    let str = r#"
ContextMap {
  ShoppingCartContext -> MallContext;
  ShoppingCartContext <-> MallContext;
}
"#;
    let context_map = MirTransform::mir(str).unwrap();

    assert_eq!(context_map.contexts.len(), 2);
    assert_eq!(context_map.relations.len(), 2);
  }

  #[test]
  fn bounded_context_out_context_map() {
    let str = r#"
ContextMap {
  ShoppingCartContext -> MallContext;
  ShoppingCartContext <-> MallContext;
}

Context ShoppingCartContext {

}

Context OrderContext {

}
"#;
    let context_map = MirTransform::mir(str).unwrap();

    assert_eq!(context_map.contexts.len(), 3);
    assert_eq!(context_map.relations.len(), 2);
  }

  #[test]
  fn mir_rel() {
    let str = r#"
ContextMap {
  ShoppingCartContext [acl] -> MallContext [acl];
}
"#;
    let context_map = MirTransform::mir(str).unwrap();

    assert_eq!(context_map.contexts.len(), 2);
    assert_eq!(context_map.relations, vec![
      ContextRelation {
        source: "ShoppingCartContext".to_string(),
        target: "MallContext".to_string(),
        connection_type: PositiveDirected,
        source_type: vec![ContextRelationType::AntiCorruptionLayer],
        target_type: vec![ContextRelationType::AntiCorruptionLayer],
      }]);
  }

  #[test]
  fn aggregate_use_entity() {
    let str = r#"
ContextMap {
  ShoppingCartContext [acl] -> MallContext [acl];
}

Context ShoppingCartContext {
  Aggregate ShoppingCart;
}

Aggregate ShoppingCart {
  Entity Shopping;
}

Entity Shopping {
  Struct {
    id: String;
  }
}
"#;
    let context_map = MirTransform::mir(str).unwrap();

    assert_eq!(context_map.contexts, vec![
      BoundedContext {
        name: "MallContext".to_string(),
        aggregates: vec![],
      },
      BoundedContext {
        name: "ShoppingCartContext".to_string(),
        aggregates: vec![
          Aggregate {
            name: "ShoppingCart".to_string(),
            description: "".to_string(),
            entities: vec![
              Entity {
                name: "Shopping".to_string(),
                description: "".to_string(),
                is_aggregate_root: false,
                identify: Field {
                  name: "".to_string(),
                  initializer: None,
                  type_type: "".to_string(),
                },
                fields: vec![],
              }
            ],
          }],
      },
    ]);
  }
}
