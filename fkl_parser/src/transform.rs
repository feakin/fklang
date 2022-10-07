use std::collections::HashMap;

use indexmap::IndexMap;

use crate::{ContextMap, mir, ParseError};
use crate::mir::{BoundedContext, ConnectionDirection, ContextRelation, ContextRelationType, Entity, Field, ValueObject};
use crate::mir::implementation::{HttpEndpoint, Implementation, Request, Response};
use crate::mir::implementation::http_api_impl::HttpApiImpl;
use crate::mir::tactic::aggregate::Aggregate;
use crate::parser::{ast, parse as ast_parse};
use crate::parser::ast::{AggregateDecl, BoundedContextDecl, EntityDecl, FklDeclaration, ImplementationDecl, RelationDirection, VariableDefinition};

#[derive(Debug, PartialEq, Eq)]
pub struct MirTransform {
  pub context_map_name: String,
  pub contexts: IndexMap<String, BoundedContext>,
  // pub contexts: HashMap<String, BoundedContext>,
  pub relations: Vec<mir::ContextRelation>,
  pub aggregates: HashMap<String, Aggregate>,
  pub entities: IndexMap<String, Entity>,
  pub value_objects: IndexMap<String, ValueObject>,
  pub implementations: Vec<HttpApiImpl>,
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
      implementations: vec![],
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
      implementations: transform.implementations.into_iter()
        .map(|impl_| Implementation::PublishHttpApi(impl_))
        .collect(),
    })
  }

  fn update_aggregates(&mut self) -> Vec<BoundedContext> {
    let mut contexts = vec![];

    self.aggregates.clone().iter().for_each(|(name, aggregate)| {
      aggregate.entities.iter().for_each(|entity| {
        if let Some(exist_entity) = self.entities.get(&entity.name) {
          let aggregate = self.aggregates.get_mut(name).unwrap();
          aggregate.entities.remove(aggregate.entities.iter().position(|e| e.name == entity.name).unwrap());
          aggregate.entities.push(exist_entity.clone());
        }
      });
    });

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

          self.relations = context_map.relations.iter().map(|relation| Self::transform_relation(&relation)).collect();
        }
        FklDeclaration::BoundedContext(decl) => {
          let context = Self::transform_bounded_context(&decl);
          self.contexts.insert(decl.name.clone(), context);
        }
        FklDeclaration::Domain(_) => {}
        FklDeclaration::Aggregate(decl) => {
          let aggregate = self.transform_aggregate(&decl);
          self.aggregates.insert(decl.name.clone(), aggregate);
        }
        FklDeclaration::DomainService(_) => {}
        FklDeclaration::ApplicationService(_) => {}
        FklDeclaration::Entity(decl) => {
          let entity = self.transform_entity(&decl);
          self.entities.insert(decl.name.clone(), entity);
        }
        FklDeclaration::ValueObject(_) => {}
        FklDeclaration::Component(_) => {}
        FklDeclaration::Implementation(implementation) => {
          let api_impl = self.transform_implementation(implementation);
          self.implementations.push(api_impl);
        }
        FklDeclaration::Struct(_) => {}
      }
    });
  }

  fn transform_relation(relation: &ast::ContextRelation) -> ContextRelation {
    mir::ContextRelation {
      source: relation.source.clone(),
      target: relation.target.clone(),
      connection_type: transform_connection(&relation.direction),
      source_type: ContextRelationType::list(&relation.source_types),
      target_type: ContextRelationType::list(&relation.target_types),
    }
  }

  fn transform_bounded_context(context_decl: &&BoundedContextDecl) -> BoundedContext {
    let mut context = mir::BoundedContext::new(&context_decl.name);
    context.aggregates = context_decl.used_domain_objects.iter().map(|domain_object| {
      Aggregate::new(&domain_object.name.clone())
    }).collect();

    context
  }

  fn transform_aggregate(&mut self, decl: &AggregateDecl) -> mir::Aggregate {
    let mut aggregate = mir::Aggregate::new(&decl.name);
    aggregate.entities = decl.used_domain_objects.iter().map(|domain_object| {
      Entity::new(&domain_object.name)
    }).collect();

    aggregate
  }

  fn transform_entity(&mut self, decl: &EntityDecl) -> mir::Entity {
    Entity {
      name: decl.name.clone(),
      is_aggregate_root: decl.is_aggregate_root,
      description: decl.inline_doc.clone(),
      fields: decl.fields.iter().map(|field| Self::transform_field(field)).collect(),
      identify: Self::transform_field(&decl.identify),
    }
  }

  fn transform_field(field: &VariableDefinition) -> Field {
    Field {
      initializer: field.initializer.clone(),
      name: field.name.clone(),
      type_type: field.type_type.clone(),
    }
  }

  fn transform_implementation(&mut self, implementation: &ImplementationDecl) -> HttpApiImpl {
    let mut http_api_impl = HttpApiImpl::new(implementation.name.clone());
    http_api_impl.endpoints = implementation.endpoints.iter().map(|endpoint_decl| {
      let mut endpoint = HttpEndpoint::new(endpoint_decl.name.clone());
      endpoint.path = endpoint_decl.uri.clone();
      endpoint.method = endpoint_decl.method.clone();
      if let Some(decl) = &endpoint_decl.response {
        endpoint.response = Some(Response {
          name: decl.name.clone(),
          post_validate: None,
        });
      }

      if let Some(decl) = &endpoint_decl.request {
        endpoint.request = Some(Request {
          name: decl.name.clone(),
          pre_validate: None,
        });
      }


      endpoint
    }).collect();

    http_api_impl
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
  use crate::mir::implementation::{HttpEndpoint, Implementation, Response};
  use crate::mir::implementation::http_api_impl::HttpApiImpl;
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
                fields: vec![
                  Field { name: "id".to_string(), initializer: None, type_type: "String".to_string() }
                ],
              }
            ],
          }],
      },
    ]);
  }

  #[test]
  fn impl_support() {
    let str = r#"
impl CinemaCreatedEvent {
  endpoint {
    GET "/book/{id}";
    authorization: Basic admin admin;
    response: Cinema;
  }
}
"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.implementations[0], Implementation::PublishHttpApi(HttpApiImpl {
      name: "CinemaCreatedEvent".to_string(),
      target_aggregate: "".to_string(),
      target_entity: "".to_string(),
      qualified: "".to_string(),
      endpoints: vec![
        HttpEndpoint {
          name: "".to_string(),
          description: "".to_string(),
          path: "/book/{id}".to_string(),
          method: "GET".to_string(),
          request: None,
          response: Some(Response {
            name: "Cinema".to_string(),
            post_validate: None,
          }),
        }
      ],
    }
    ));
  }
}
