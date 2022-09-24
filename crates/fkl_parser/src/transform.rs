use indexmap::IndexMap;

use crate::{ContextMap, mir, ParseError};
use crate::mir::{BoundedContext, ConnectionDirection, ContextRelationType};
use crate::parser::parse as ast_parse;
use crate::parser::ast::{FklDeclaration, RelationDirection};

pub struct Transform {
  pub contexts: IndexMap<String, BoundedContext>,
  pub relations: Vec<mir::ContextRelation>,
}

impl Transform {
  pub fn mir(str: &str) -> Result<ContextMap, ParseError> {
    let mut transform = Transform {
      contexts: Default::default(),
      relations: vec![],
    };

    match ast_parse(str) {
      Ok(decls) => {
        decls.iter().for_each(|decl| {
          match decl {
            FklDeclaration::None => {}
            FklDeclaration::ContextMap(context_map) => {
              context_map.contexts.iter().for_each(|context| {
                let bounded_context = mir::BoundedContext::new(&context.name);
                transform.contexts.insert(bounded_context.name.clone(), bounded_context);
              });

              context_map.relations.iter().for_each(|relation| {
                let rel = mir::ContextRelation {
                  source: relation.source.clone(),
                  target: relation.target.clone(),
                  connection_type: transform_connection(&relation.direction),
                  source_type: ContextRelationType::list(&relation.source_types),
                  target_type: ContextRelationType::list(&relation.target_types),
                };
                transform.relations.push(rel);
              });
            }
            FklDeclaration::BoundedContext(bc) => {
              let bounded_context = mir::BoundedContext::new(&bc.name);
              transform.contexts.insert(bounded_context.name.clone(), bounded_context);
            }
            FklDeclaration::Domain(_) => {}
            FklDeclaration::Aggregate(_) => {}
            FklDeclaration::DomainService(_) => {}
            FklDeclaration::ApplicationService(_) => {}
            FklDeclaration::Entity(_) => {}
            FklDeclaration::ValueObject(_) => {}
            FklDeclaration::Component(_) => {}
          }
        });
      }
      Err(e) => return Err(e),
    };

    Ok(ContextMap {
      name: "".to_string(),
      state: Default::default(),
      contexts: transform.contexts.values().map(|context| context.clone()).collect(),
      relations: transform.relations,
    })
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
  use crate::mir::ConnectionDirection::PositiveDirected;
  use crate::mir::{ContextRelation, ContextRelationType};
  use crate::transform::Transform;

  #[test]
  fn basic_mir() {
    let str = r#"
ContextMap {
  ShoppingCartContext -> MallContext;
  ShoppingCartContext <-> MallContext;
}
"#;
    let context_map = Transform::mir(str).unwrap();

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
    let context_map = Transform::mir(str).unwrap();

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
    let context_map = Transform::mir(str).unwrap();

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
