use std::collections::HashMap;
use indexmap::IndexMap;

use crate::{ContextMap, mir, ParseError};
use crate::mir::{BoundedContext, ConnectionDirection, ContextRelationType};
use crate::mir::tactic::aggregate::Aggregate;
use crate::parser::ast::{AggregateDecl, FklDeclaration, RelationDirection};
use crate::parser::parse as ast_parse;

pub struct MirTransform {
  pub context_map_name: String,
  pub contexts: IndexMap<String, BoundedContext>,
  // pub contexts: HashMap<String, BoundedContext>,
  pub aggregates: IndexMap<String, Aggregate>,
  pub relations: Vec<mir::ContextRelation>,
}

impl MirTransform {
  pub fn mir(str: &str) -> Result<ContextMap, ParseError> {
    let mut transform = MirTransform {
      context_map_name: "".to_string(),
      contexts: Default::default(),
      aggregates: Default::default(),
      relations: vec![],
    };

    match ast_parse(str) {
      Ok(decls) => {
        transform.lower_decls(decls);
      }
      Err(e) => return Err(e),
    };

    transform.build_context_map();

    Ok(ContextMap {
      name: transform.context_map_name,
      state: Default::default(),
      contexts: transform.contexts.values().map(|context| context.clone()).collect(),
      relations: transform.relations,
    })
  }

  fn build_context_map(&self) -> Vec<BoundedContext> {
    let mut contexts = vec![];

    self.contexts.values().for_each(|context| {
      let mut context = BoundedContext::new(&context.name);

      for aggregate in context.aggregates.clone() {
        if let Some(agg) = self.aggregates.get(&aggregate.name) {
          context.aggregates.push(agg.clone());
        } else {
          context.aggregates.push(aggregate.clone())
        }
      }
    });

    contexts
  }

  fn lower_decls(&mut self, decls: Vec<FklDeclaration>) {
    decls.iter().for_each(|declaration| {
      match declaration {
        FklDeclaration::None => {}
        FklDeclaration::ContextMap(context_map) => {
          self.context_map_name = context_map.name.name.clone();

          context_map.contexts.iter().for_each(|context| {
            let bounded_context = mir::BoundedContext::new(&context.name);
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
          let mut bounded_context = mir::BoundedContext::new(&bc.name);
          bc.use_domain_objects.iter().for_each(|domain_object| {
            let aggregate = Aggregate::new(&domain_object.name);
            bounded_context.aggregates.push(aggregate);
          });
          self.contexts.insert(bounded_context.name.clone(), bounded_context);
        }
        FklDeclaration::Domain(_) => {}
        FklDeclaration::Aggregate(decl) => {
          self.transform_aggregate(&decl);
        }
        FklDeclaration::DomainService(_) => {}
        FklDeclaration::ApplicationService(_) => {}
        FklDeclaration::Entity(_) => {}
        FklDeclaration::ValueObject(_) => {}
        FklDeclaration::Component(_) => {}
      }
    });
  }

  fn transform_aggregate(&mut self, decl: &AggregateDecl) -> mir::Aggregate {
    let mut aggregate = mir::Aggregate::new(&decl.name);
    self.aggregates.insert(aggregate.name.clone(), aggregate.clone());
    // decl.entities.iter().for_each(|entity| {
    // });

    aggregate
  }

  // fn transform_entity(decl: &EntityDecl) -> mir::Entity {
  //   let entity = mir::Entity::new(&decl.name);
  //   entity
  // }
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
  use crate::mir::{ContextRelation, ContextRelationType};
  use crate::mir::ConnectionDirection::PositiveDirected;
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
}
