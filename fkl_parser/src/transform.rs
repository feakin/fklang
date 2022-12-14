use std::collections::HashMap;

use indexmap::IndexMap;

use fkl_mir::{BoundedContext, ConnectionDirection, ContextRelation, ContextRelationType, Datasource, Entity, Field, Flow, HttpMethod, Layer, LayeredArchitecture, LayerRelation, MethodCall, MySqlDatasource, PostgresDatasource, Step, ValueObject};
use fkl_mir as mir;
use fkl_mir::authorization::HttpAuthorization;
use fkl_mir::implementation::{HttpEndpoint, Implementation, Request, Response};
use fkl_mir::implementation::http_api_impl::HttpApiImpl;
use fkl_mir::tactic::aggregate::Aggregate;

use crate::{ContextMap, ParseError};
use crate::parser::{ast, parse as ast_parse};
use crate::parser::ast::{AggregateDecl, BoundedContextDecl, CustomDecl, DatasourceDecl, EndpointDecl, EntityDecl, EnvDecl, FklDeclaration, FlowDecl, ImplementationDecl, ImplementationTargetType, LayeredDecl, MethodCallDecl, RelationDirection, ServerDecl, SourceSetsDecl, StepDecl, VariableDefinition};

#[derive(Debug, PartialEq, Eq)]
pub struct MirTransform {
  pub context_map_name: String,
  pub contexts: IndexMap<String, BoundedContext>,
  pub relations: Vec<ContextRelation>,
  pub aggregates: HashMap<String, Aggregate>,
  pub entities: IndexMap<String, Entity>,
  pub value_objects: IndexMap<String, ValueObject>,
  pub implementations: Vec<HttpApiImpl>,
  pub layered: Option<LayeredArchitecture>,
  pub source_sets: Option<fkl_mir::SourceSets>,
  pub envs: Vec<fkl_mir::Environment>,
  pub structs: HashMap<String, fkl_mir::Struct>,
}

impl MirTransform {
  // todo: refactor to symbol table
  pub fn mir(str: &str) -> Result<ContextMap, ParseError> {
    let mut transform = MirTransform {
      context_map_name: "".to_string(),
      contexts: Default::default(),
      aggregates: Default::default(),
      relations: vec![],
      entities: Default::default(),
      value_objects: Default::default(),
      implementations: vec![],
      layered: Default::default(),
      source_sets: None,
      envs: vec![],
      structs: Default::default()
    };

    match ast_parse(str) {
      Ok(decls) => {
        transform.lower_decls(decls);
      }
      Err(e) => return Err(e),
    };

    let contexts = transform.update_aggregates();

    // todo: add custom struct

    Ok(ContextMap {
      name: transform.context_map_name,
      state: Default::default(),
      contexts,
      relations: transform.relations,
      implementations: transform.implementations.into_iter()
        .map(|impl_| Implementation::PublishHttpApi(impl_))
        .collect(),
      layered: transform.layered,
      source_sets: transform.source_sets,
      envs: transform.envs,
      structs: transform.structs,
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
            let bounded_context = self.transform_bounded_context(&context_decl);

            self.contexts.insert(bounded_context.name.clone(), bounded_context);
          });

          self.relations = context_map.relations.iter().map(|relation| Self::transform_relation(&relation)).collect();
        }
        FklDeclaration::BoundedContext(decl) => {
          let context = self.transform_bounded_context(&decl);
          self.contexts.insert(decl.name.clone(), context);
        }
        FklDeclaration::Aggregate(decl) => {
          let aggregate = self.transform_aggregate(&decl);
          self.aggregates.insert(decl.name.clone(), aggregate);
        }
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
        FklDeclaration::Struct(decl) => {
          let fields: Vec<Field> = decl.fields.iter().map(|field| Self::transform_field(field)).collect();
          self.structs.insert(decl.name.clone(), fkl_mir::Struct {
            name: decl.name.clone(),
            fields,
          });
        }
        FklDeclaration::Layered(decl) => {
          self.layered = Some(self.transform_layered(&decl));
        }
        FklDeclaration::SourceSets(decl) => {
          self.source_sets = Some(self.transform_source_sets(&decl));
        }
        FklDeclaration::Include(_include) => {
          // todo: resolve include with DAG
        }
        FklDeclaration::Env(decl) => {
          self.envs.push(self.transform_environment(&decl));
        }
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

  fn transform_bounded_context(&self, context_decl: &BoundedContextDecl) -> BoundedContext {
    let mut context = mir::BoundedContext::new(&context_decl.name);
    context.aggregates = context_decl.used_domain_objects.iter().map(|domain_object| {
      Aggregate::new(&domain_object.name.clone())
    }).collect();

    let from_inside: Vec<Aggregate> = context_decl.aggregates.iter().map(|decl| {
      let mut entities: Vec<Entity> = decl.entities.iter().map(|entity| {
        self.transform_entity(entity)
      }).collect();

      let used_entities: Vec<Entity> = decl.used_domain_objects.iter().map(|domain_object| {
        Entity::new(&domain_object.name)
      }).collect();

      entities.extend(used_entities);

      Aggregate {
        name: decl.name.clone(),
        description: "".to_string(),
        entities: entities,
      }
    }).collect();

    context.aggregates.extend(from_inside);

    context
  }

  fn transform_aggregate(&mut self, decl: &AggregateDecl) -> mir::Aggregate {
    let mut aggregate = mir::Aggregate::new(&decl.name);
    aggregate.entities = decl.used_domain_objects.iter().map(|domain_object| {
      Entity::new(&domain_object.name)
    }).collect();

    decl.entities.iter().for_each(|entity| {
      aggregate.entities.push(self.transform_entity(entity));
    });

    aggregate
  }

  fn transform_entity(&self, decl: &EntityDecl) -> mir::Entity {
    Entity {
      name: decl.name.clone(),
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
    http_api_impl.endpoint = Self::transform_endpoint(&implementation.endpoint);

    http_api_impl.flow = if let Some(flow) = &implementation.flow {
      Some(self.transform_flow(&flow))
    } else {
      None
    };

    if let Some(target) = &implementation.target {
      match target.target_type {
        ImplementationTargetType::None => {}
        ImplementationTargetType::Aggregate => {
          http_api_impl.target_aggregate = target.name.clone();
        }
        ImplementationTargetType::Entity => {
          http_api_impl.target_entity = target.name.clone()
        }
        ImplementationTargetType::ValueObject => {}
      }
    }

    http_api_impl
  }

  fn transform_flow(&mut self, flow_decl: &FlowDecl) -> Flow {
    let mut flow = Flow::default();
    flow.steps = flow_decl.steps.iter().map(|step_decl| {
      match step_decl {
        StepDecl::MethodCall(call) => {
          let mut method_call = MethodCall::new(call.name.clone());
          method_call.method = call.method.clone();
          method_call.object = call.object.clone();
          method_call.parameters = self.transform_variables(&call.arguments);
          method_call.return_type = self.transform_return_type(&call);

          Step::MethodCall(method_call)
        }
        StepDecl::Message(msg) => {
          let mut message = mir::Message::default();
          message.from = msg.from.clone();
          message.topic = msg.topic.clone();
          message.message = msg.message.clone();

          Step::Message(message)
        }
      }
    }).collect();

    flow
  }

  fn transform_endpoint(endpoint_decl: &EndpointDecl) -> HttpEndpoint {
    let mut endpoint = HttpEndpoint::new(endpoint_decl.name.clone());
    endpoint.method = HttpMethod::from(&endpoint_decl.method);
    endpoint.path = endpoint_decl.uri.clone();
    if let Some(decl) = &endpoint_decl.response {
      endpoint.response = Some(Response {
        name: decl.name.clone(),
        post_validate: None,
      });
    }

    if let Some(decl) = &endpoint_decl.authorization {
      let authorization = HttpAuthorization::from(&decl.auth_type, decl.username.clone(), decl.password.clone());
      endpoint.auth = Some(authorization);
    }

    if let Some(decl) = &endpoint_decl.request {
      endpoint.request = Some(Request {
        name: decl.name.clone(),
        pre_validate: None,
      });
    }


    endpoint
  }

  fn transform_return_type(&mut self, call: &&MethodCallDecl) -> Option<mir::VariableDefinition> {
    match &call.return_type {
      None => None,
      Some(var) => {
        Some(self.transform_variables(&vec![var.clone()])[0].clone())
      }
    }
  }

  fn transform_variables(&self, vars: &Vec<ast::VariableDefinition>) -> Vec<mir::VariableDefinition> {
    vars.iter().map(|var| {
      mir::VariableDefinition {
        name: var.name.clone(),
        type_type: var.type_type.clone(),
        initializer: var.initializer.clone(),
      }
    }).collect()
  }

  fn transform_layered(&self, decl: &LayeredDecl) -> LayeredArchitecture {
    let mut layered = LayeredArchitecture::default();

    layered.name = decl.name.clone();
    layered.relations = decl.dependencies.iter().map(|rule| {
      let mut dependency_rule = LayerRelation::default();
      dependency_rule.source = rule.source.clone();
      dependency_rule.target = rule.target.clone();

      dependency_rule
    }).collect();

    layered.layers = decl.layers.iter().map(|layer| {
      Layer {
        name: layer.name.clone(),
        package: layer.package.clone(),
      }
    }).collect();

    layered
  }

  fn transform_source_sets(&self, decl: &SourceSetsDecl) -> mir::SourceSets {
    let mut source_sets = mir::SourceSets::default();
    source_sets.name = decl.name.clone();
    source_sets.source_sets = decl.source_sets.iter().map(|source_set| {
      let mut set = mir::SourceSet {
        name: source_set.name.clone(),
        description: "".to_string(),
        parser: "".to_string(),
        extension: "".to_string(),
        src_dirs: vec![],
        source_set_type: Default::default(),
      };


      source_set.attributes.iter().for_each(|attr| {
        match attr.key.as_str() {
          "description" => set.description = attr.value[0].clone(),
          "parser" => set.parser = attr.value[0].clone(),
          "extension" => set.extension = attr.value[0].clone(),
          "srcDir" => set.src_dirs = attr.value.clone(),
          &_ => {
            println!("Unknown attribute {}", attr.key);
          }
        }
      });

      set
    }).collect();

    source_sets
  }

  fn transform_environment(&self, decl: &EnvDecl) -> mir::Environment {
    let mut environment = mir::Environment::default();
    environment.name = decl.name.clone();

    if let Some(ds) = &decl.datasource {
      environment.datasources.push(self.transform_datasource(&ds));
    }

    if let Some(orm) = &decl.server {
      environment.server = self.transform_server_config(&orm);
    }

    environment.customs = decl.customs.iter().map(|custom| {
      self.transform_custom_env(&custom)
    }).collect();

    environment
  }

  fn transform_datasource(&self, decl: &DatasourceDecl) -> mir::Datasource {
    if !decl.url.is_empty() {
      return Datasource::from(&decl.url).unwrap();
    }

    let driver: &str = &decl.driver;
    match driver {
      "mysql" => {
        Datasource::MySql(MySqlDatasource {
          host: decl.host.clone(),
          port: decl.port.clone().parse().unwrap(),
          username: decl.username.clone(),
          password: decl.password.clone(),
          database: decl.database.clone(),
        })
      }
      "postgresql" => {
        Datasource::Postgres(PostgresDatasource {
          host: decl.host.clone(),
          port: decl.port.clone().parse().unwrap(),
          username: decl.username.clone(),
          password: decl.password.clone(),
          database: decl.database.clone(),
        })
      }
      _ => {
        panic!("Unknown driver {}", driver);
      }
    }
  }

  fn transform_server_config(&self, decl: &ServerDecl) -> mir::ServerConfig {
    let mut server = mir::ServerConfig::default();
    server.port = decl.port.clone();

    server
  }

  fn transform_custom_env(&self, decl: &CustomDecl) -> mir::CustomEnv {
    let mut custom = mir::CustomEnv::default();
    custom.name = decl.name.clone();
    custom.attrs = decl.attributes.iter().map(|attr| {
      mir::VariableDefinition {
        name: attr.key.clone(),
        type_type: "".to_string(),
        initializer: Some(attr.value[0].clone()),
      }
    }).collect();

    custom
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
  use fkl_mir::{Aggregate, BoundedContext, ContextRelation, ContextRelationType, CustomEnv, Entity, Environment, Flow, HttpMethod, Layer, LayeredArchitecture, LayerRelation, MethodCall, PostgresDatasource, ServerConfig, SourceSet, SourceSets, Step, VariableDefinition};
  use fkl_mir::authorization::HttpAuthorization;
  use fkl_mir::ConnectionDirection::PositiveDirected;
  use fkl_mir::Datasource::Postgres;
  use fkl_mir::implementation::{HttpEndpoint, Implementation, Response};
  use fkl_mir::implementation::http_api_impl::HttpApiImpl;
  use fkl_mir::tactic::block::Field;

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
  aggregate: Cinema;
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
      target_aggregate: "Cinema".to_string(),
      target_entity: "".to_string(),
      qualified: "".to_string(),
      endpoint: HttpEndpoint {
        name: "".to_string(),
        description: "".to_string(),
        path: "/book/{id}".to_string(),
        auth: Some(HttpAuthorization::Basic("admin".to_string(), "admin".to_string())),
        method: HttpMethod::GET,
        request: None,
        response: Some(Response {
          name: "Cinema".to_string(),
          post_validate: None,
        }),
      },
      flow: None,
    }
    ));
  }

  #[test]
  fn impl_support_flow() {
    let str = r#"
impl CinemaCreatedEvent {
  endpoint {
    GET "/book/{id}";
    authorization: Basic admin admin;
    response: Cinema;
  }

   flow {
      via UserRepository::getUserById receive user: User
      via UserRepository::save(user: User) receive user: User;
   }
}"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.implementations[0], Implementation::PublishHttpApi(HttpApiImpl {
      name: "CinemaCreatedEvent".to_string(),
      target_aggregate: "".to_string(),
      target_entity: "".to_string(),
      qualified: "".to_string(),
      endpoint: HttpEndpoint {
        name: "".to_string(),
        description: "".to_string(),
        path: "/book/{id}".to_string(),
        auth: Some(HttpAuthorization::Basic("admin".to_string(), "admin".to_string())),
        method: HttpMethod::GET,
        request: None,
        response: Some(Response {
          name: "Cinema".to_string(),
          post_validate: None,
        }),
      },
      flow: Some(Flow {
        inline_doc: "".to_string(),
        steps: vec![
          Step::MethodCall(MethodCall {
            name: "".to_string(),
            object: "UserRepository".to_string(),
            method: "getUserById".to_string(),
            parameters: vec![],
            return_type: Some(VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
            }),
          }),
          Step::MethodCall(MethodCall {
            name: "".to_string(),
            object: "UserRepository".to_string(),
            method: "save".to_string(),
            parameters: vec![VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
            }],
            return_type: Some(VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
            }),
          })],
      }),
    }
    ));
  }

  #[test]
  fn lower_layered() {
    let str = r#"layered DDD {
  dependency {
    "rest" -> "application"
    "rest" -> "domain"
    "domain" -> "application"
    "application" -> "infrastructure"
    "rest" -> "infrastructure"
  }
  layer rest {
     package: "com.example.book";
  }
  layer domain {
     package: "com.example.domain";
  }
  layer application {
    package: "com.example.application";
  }
  layer infrastructure {
    package: "com.example.infrastructure";
  }
}"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.layered, Some(LayeredArchitecture {
      name: "DDD".to_string(),
      layers: vec![
        Layer {
          name: "rest".to_string(),
          package: "com.example.book".to_string(),
        },
        Layer {
          name: "domain".to_string(),
          package: "com.example.domain".to_string(),
        },
        Layer {
          name: "application".to_string(),
          package: "com.example.application".to_string(),
        },
        Layer {
          name: "infrastructure".to_string(),
          package: "com.example.infrastructure".to_string(),
        },
      ],
      relations: vec![
        LayerRelation {
          source: "rest".to_string(),
          target: "application".to_string(),
        },
        LayerRelation {
          source: "rest".to_string(),
          target: "domain".to_string(),
        },
        LayerRelation {
          source: "domain".to_string(),
          target: "application".to_string(),
        },
        LayerRelation {
          source: "application".to_string(),
          target: "infrastructure".to_string(),
        },
        LayerRelation {
          source: "rest".to_string(),
          target: "infrastructure".to_string(),
        },
      ],
      description: "".to_string(),
    }));
  }

  #[test]
  fn mir_source_set() {
    let str = r#"SourceSet sourceSet {
  feakin {
    srcDir: ["src/main/resources/uml"]
  }
  puml {
    parser: "PlantUML"
    srcDir: ["src/main/resources/uml"]
  }
}"#;
    let context_map = MirTransform::mir(str).unwrap();

    assert_eq!(context_map.source_sets, Some(
      SourceSets {
        name: "sourceSet".to_string(),
        source_sets: vec![
          SourceSet {
            name: "feakin".to_string(),
            parser: "".to_string(),
            extension: "".to_string(),
            src_dirs: vec!["src/main/resources/uml".to_string()],
            description: "".to_string(),
            source_set_type: Default::default(),
          },
          SourceSet {
            name: "puml".to_string(),
            parser: "PlantUML".to_string(),
            extension: "".to_string(),
            src_dirs: vec!["src/main/resources/uml".to_string()],
            description: "".to_string(),
            source_set_type: Default::default(),
          },
        ],
      }
    ));
  }

  #[test]
  fn env_database_server() {
    let str = r#"env Local {
  datasource {
    driver: postgresql
    host: "localhost"
    port: 5432
    database: "test"
  }
  server {
    port: 9090;
  }
}"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.envs, vec![
      Environment {
        name: "Local".to_string(),
        datasources: vec![Postgres(PostgresDatasource {
          host: "localhost".to_string(),
          port: 5432,
          username: "".to_string(),
          password: "".to_string(),
          database: "test".to_string(),
        })],
        server: ServerConfig {
          port: 9090,
        },
        customs: vec![],
      }
    ]);
  }

  #[test]
  fn custom_env() {
    let str = r#"env Local {
  kafka {
    host: "localhost"
    port: 9092
  }
}"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.envs, vec![
      Environment {
        name: "Local".to_string(),
        datasources: vec![],
        server: ServerConfig {
          port: 8899,
        },
        customs: vec![
          CustomEnv {
            name: "kafka".to_string(),
            attrs: vec![
              VariableDefinition {
                name: "host".to_string(),
                type_type: "".to_string(),
                initializer: Some("localhost".to_string()),
              },
              VariableDefinition {
                name: "port".to_string(),
                type_type: "".to_string(),
                initializer: Some("9092".to_string()),
              }],
          }
        ],
      }
    ]);
  }

  #[test]
  fn empty_struct() {
    let str = r#"ContextMap Movie {
  Context Movie {
    Aggregate Movie {
      Entity Movie, Actor, Publisher;
    }
  }
}"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.contexts, vec![
      BoundedContext {
        name: "Movie".to_string(),
        aggregates: vec![
          Aggregate {
            name: "Movie".to_string(),
            entities: vec![
              Entity {
                name: "Movie".to_string(),
                fields: vec![],
                description: "".to_string(),
                identify: Default::default(),
              },
              Entity {
                name: "Actor".to_string(),
                fields: vec![],
                description: "".to_string(),
                identify: Default::default(),
              },
              Entity {
                name: "Publisher".to_string(),
                fields: vec![],
                description: "".to_string(),
                identify: Default::default(),
              },
            ],
            description: "".to_string(),
          },
        ],
      },
    ]);
  }

  #[test]
  fn nested_aggregate() {
    let str = r#"ContextMap architecture {
    Context analyze {
        Aggregate ArchSystem {
            Struct {
                id: String;
                name: String;
            }

            Entity ArchComponent {
                Struct {
                    name: String;
                    type: ArchComponentType
                }
            }
        }
    }
}"#;

    let context_map = MirTransform::mir(str).unwrap();
    assert_eq!(context_map.contexts, vec![
      BoundedContext {
        name: "analyze".to_string(),
        aggregates: vec![Aggregate {
          name: "ArchSystem".to_string(),
          description: "".to_string(),
          entities: vec![
            Entity {
              name: "ArchSystem".to_string(),
              description: "".to_string(),
              identify: Field { name: "".to_string(), initializer: None, type_type: "".to_string() },
              fields: vec![
                Field { name: "id".to_string(), initializer: None, type_type: "String".to_string() },
                Field { name: "name".to_string(), initializer: None, type_type: "String".to_string() }],
            },
            Entity {
              name: "ArchComponent".to_string(),
              description: "".to_string(),
              identify: Field { name: "".to_string(), initializer: None, type_type: "".to_string() },
              fields: vec![
                Field { name: "name".to_string(), initializer: None, type_type: "String".to_string() },
                Field { name: "type".to_string(), initializer: None, type_type: "ArchComponentType".to_string() }],
            }],
        }],
      }
    ]);
  }
}
