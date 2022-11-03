use std::collections::HashMap;
use std::hash::Hash;

use indexmap::IndexMap;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};

use crate::default_config;
use crate::parser::ast::{AggregateDecl, AttributeDefinition, AuthorizationDecl, BoundedContextDecl, ComponentDecl, ContextMapDecl, ContextRelation, CustomDecl, DatasourceDecl, DomainEventDecl, EndpointDecl, EntityDecl, EnvDecl, FklDeclaration, FlowDecl, HttpRequestDecl, HttpResponseDecl, Identifier, ImplementationDecl, ImplementationTarget, ImplementationTargetType, IncludeDecl, LayerDecl, LayeredDecl, LayerRelationDecl, Loc, MessageDecl, MethodCallDecl, RelationDirection, ServerDecl, SourceSetDecl, SourceSetsDecl, StepDecl, StructDecl, UsedDomainObject, ValueObjectDecl, VariableDefinition};
use crate::parser::parse_result::{ParseError, ParseResult};
use crate::pest::Parser;

#[derive(Parser)]
#[grammar = "parser/fkl.pest"]
pub struct FklParser;

pub fn parse(code: &str) -> ParseResult<Vec<FklDeclaration>> {
  match inner_parse(code) {
    Err(e) => {
      // match &e.variant {
      //   ErrorVariant::CustomError { message } => {
      //     println!("Custom error: {}", message);
      //   }
      //   ErrorVariant::ParsingError { positives, negatives } => {
      //     positives.iter().for_each(|p| println!("Positive: {:?}", p));
      //     negatives.iter().for_each(|p| println!("Negative: {:?}", p));
      //     println!("Parsing error: positives: {:?}, negatives: {:?}", positives, negatives);
      //   }
      // };

      // let fancy_e = e.renamed_rules(|rule| {
      //   match *rule {
      //     Rule::EOI => "end of input".to_string(),
      //     Rule::inline_doc => "`\"\"\"`".to_string(),
      //     Rule::layer_decl => "`layer`".to_string(),
      //     Rule::dependency_decl => "`dependency`".to_string(),
      //     _ => "".to_string(),
      //   }
      // });
      // return Err(ParseError::msg(fancy_e));
      return Err(ParseError::msg(e.to_string()));
    }
    Ok(pairs) => {
      Ok(consume_declarations(pairs))
    }
  }
}

fn inner_parse(code: &str) -> Result<Pairs<Rule>, Error<Rule>> {
  FklParser::parse(Rule::declarations, code)
}

fn consume_declarations(pairs: Pairs<Rule>) -> Vec<FklDeclaration> {
  pairs.filter(|pair| {
    return pair.as_rule() == Rule::declaration;
  }).map(|pair| {
    let mut decl: FklDeclaration = FklDeclaration::None;
    for p in pair.into_inner() {
      match p.as_rule() {
        Rule::context_map_decl => {
          decl = FklDeclaration::ContextMap(consume_context_map(p));
        }
        Rule::context_decl => {
          decl = FklDeclaration::BoundedContext(consume_context(p));
        }
        Rule::aggregate_decl => {
          decl = FklDeclaration::Aggregate(consume_aggregate(p));
        }
        Rule::entity_decl => {
          decl = FklDeclaration::Entity(consume_entity(p));
        }
        Rule::component_decl => {
          decl = FklDeclaration::Component(consume_component(p));
        }
        Rule::value_object_decl => {
          decl = FklDeclaration::ValueObject(consume_value_object(p));
        }
        Rule::implementation_decl => {
          decl = FklDeclaration::Implementation(consume_implementation(p));
        }
        Rule::struct_decl => {
          decl = FklDeclaration::Struct(consume_struct(p));
        }
        Rule::layered_decl => {
          decl = FklDeclaration::Layered(consume_layered(p));
        }
        Rule::source_sets_decl => {
          decl = FklDeclaration::SourceSets(consume_source_sets(p));
        }
        Rule::include_decl => {
          decl = FklDeclaration::Include(consume_include(p));
        }
        Rule::env_decl => {
          decl = FklDeclaration::Env(consume_env(p));
        }
        _ => println!("unreachable declaration rule: {:?}", p.as_rule())
      };
    }
    return decl;
  }).collect::<Vec<FklDeclaration>>()
}

fn consume_include(pair: Pair<Rule>) -> IncludeDecl {
  let mut path = String::new();
  let loc = Loc::from_pair(pair.as_span());
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::string => {
        path = p.as_str().to_string();
      }
      _ => println!("unreachable content rule: {:?}", p.as_rule())
    };
  }

  return IncludeDecl { path, loc };
}

fn consume_context_map(pair: Pair<Rule>) -> ContextMapDecl {
  let mut context_decl_map: IndexMap<String, BoundedContextDecl> = IndexMap::new();
  let mut identify = Identifier::default();
  let mut relations: Vec<ContextRelation> = Vec::new();
  let span = pair.as_span().clone();

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        identify.loc = Loc::from_pair(p.as_span());
        identify.name = p.as_str().to_string();
      }
      Rule::context_decl => {
        let context_decl = consume_context(p);
        context_decl_map.insert(context_decl.name.clone(), context_decl);
      }
      Rule::context_node_rel => {
        let context_relation = consume_context_node(&mut context_decl_map, p);
        relations.push(context_relation);
      }
      _ => println!("unreachable context_map rule: {:?}", p.as_rule())
    };
  }

  // sort context map by name
  let mut contexts = context_decl_map.into_iter().map(|(_, v)| v)
    .collect::<Vec<BoundedContextDecl>>();

  contexts.sort_by(|a, b| a.name.cmp(&b.name));

  return ContextMapDecl {
    loc: Loc::from_pair(span),
    name: identify,
    contexts,
    relations,
  };
}

fn consume_context_node(context_decl_map: &mut IndexMap<String, BoundedContextDecl>, pair: Pair<Rule>) -> ContextRelation {
  let mut names: Vec<String> = vec![];
  let mut direction: RelationDirection = RelationDirection::Undirected;
  let mut source_type: Vec<String> = vec![];
  let mut target_type: Vec<String> = vec![];

  for p in pair.into_inner() {
    let loc = Loc::from_pair(p.as_span());
    match p.as_rule() {
      Rule::left_id | Rule::right_id => {
        let context_name = p.as_str().to_string();
        names.push(context_name.clone());
        context_decl_map.insert(context_name.clone(), BoundedContextDecl {
          name: context_name,
          domain_events: vec![],
          aggregates: vec![],
          used_domain_objects: vec![],
          loc,
        });
      }
      Rule::rel_symbol => {
        for p in p.into_inner() {
          match p.as_rule() {
            Rule::rs_both => {
              direction = RelationDirection::BiDirected;
            }
            Rule::rs_left_to_right => {
              direction = RelationDirection::PositiveDirected;
            }
            Rule::rs_right_to_left => {
              direction = RelationDirection::NegativeDirected;
            }
            _ => println!("unreachable entity rule: {:?}", p.as_rule())
          };
        }
      }
      Rule::left_rel_defs => {
        for p in p.into_inner() {
          if p.as_rule() == Rule::rel_defs {
            source_type = rel_defs(p);
          }
        }
      }
      Rule::right_rel_defs => {
        for p in p.into_inner() {
          if p.as_rule() == Rule::rel_defs {
            target_type = rel_defs(p);
          }
        }
      }
      _ => println!("unreachable context rel rule: {:?}", p.as_rule())
    };
  }

  let context_relation = ContextRelation {
    source: names[0].clone(),
    target: names[1].clone(),
    direction,
    source_types: source_type,
    target_types: target_type,
  };
  context_relation
}

fn rel_defs(pair: Pair<Rule>) -> Vec<String> {
  let mut types: Vec<String> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        types.push(p.as_str().to_string());
      }
      _ => println!("unreachable rel_def rule: {:?}", p.as_rule())
    };
  }

  return types;
}

fn consume_context(pair: Pair<Rule>) -> BoundedContextDecl {
  let mut context = BoundedContextDecl::default();
  context.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        context.name = p.as_str().to_string();
      }
      Rule::aggregate_decl => {
        context.aggregates.push(consume_aggregate(p));
      }
      Rule::used_domain_objects_decl => {
        let vec = consume_use_domain_object(p);
        context.used_domain_objects = [context.used_domain_objects, vec].concat();
      }
      _ => println!("unreachable context rule: {:?}", p.as_rule())
    };
  }
  return context;
}

fn consume_aggregate(pair: Pair<Rule>) -> AggregateDecl {
  let mut aggregate = AggregateDecl::default();
  aggregate.loc = Loc::from_pair(pair.as_span());
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        aggregate.name = p.as_str().to_string();
      }
      Rule::inline_doc => {
        aggregate.inline_doc = parse_inline_doc(p);
      }
      Rule::entity_decl => {
        aggregate.entities.push(consume_entity(p));
      }
      Rule::used_domain_objects_decl => {
        aggregate.used_domain_objects = [aggregate.used_domain_objects, consume_use_domain_object(p)].concat();
      }
      Rule::used_domain_event_decl => {
        aggregate.domain_events = consume_use_domain_events(p);
      }
      Rule::struct_decl => {
        let default_struct = consume_struct(p);
        let fields = default_struct.fields;
        aggregate.entities.push(EntityDecl {
          name: aggregate.name.to_string(),
          is_aggregate_root: false,
          identify: Default::default(),
          inline_doc: "".to_string(),
          fields,
          value_objects: vec![],
          loc: default_struct.loc,
        });
      }
      _ => println!("unreachable aggregate rule: {:?}", p.as_rule())
    };
  }

  return aggregate;
}

pub fn consume_use_domain_events(pair: Pair<Rule>) -> Vec<DomainEventDecl> {
  let mut domain_events: Vec<DomainEventDecl> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::event_name => {
        let loc = Loc::from_pair(p.as_span());
        domain_events.push(DomainEventDecl {
          name: p.as_str().to_string(),
          loc
        });
      }
      _ => println!("unreachable use_domain_events rule: {:?}", p.as_rule())
    };
  }

  return domain_events;
}

fn consume_entity(pair: Pair<Rule>) -> EntityDecl {
  let mut entity = EntityDecl::default();
  entity.loc = Loc::from_pair(pair.as_span());
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        entity.name = p.as_str().to_string();
      }
      Rule::constructor_decl => {
        entity.fields = consume_constructor_decl(p);
      }
      Rule::struct_decl => {
        entity.fields = consume_struct_decl(p);
      }
      Rule::inline_doc => {
        entity.inline_doc = parse_inline_doc(p);
      }
      Rule::value_object_decl => {
        entity.value_objects.push(consume_value_object(p));
      }
      _ => println!("unreachable entity rule: {:?}", p.as_rule())
    };
  }
  return entity;
}

fn consume_use_domain_object(pair: Pair<Rule>) -> Vec<UsedDomainObject> {
  let mut used_domain_objects: Vec<UsedDomainObject> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        let loc = Loc::from_pair(p.as_span());
        used_domain_objects.push(UsedDomainObject {
          name: p.as_str().to_string(),
          loc
        });
      }
      _ => println!("unreachable use_domain_object rule: {:?}", p.as_rule())
    };
  }

  used_domain_objects
}

fn consume_constructor_decl(pair: Pair<Rule>) -> Vec<VariableDefinition> {
  let mut fields: Vec<VariableDefinition> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::parameters_decl => {
        fields = consume_parameters(p);
      }
      _ => println!("unreachable constructor rule: {:?}", p.as_rule())
    };
  }
  return fields;
}

fn consume_parameters(p: Pair<Rule>) -> Vec<VariableDefinition> {
  let mut fields: Vec<VariableDefinition> = vec![];
  for p in p.into_inner() {
    match p.as_rule() {
      Rule::name_type_def => {
        fields.push(consume_parameter(p));
      }
      _ => println!("unreachable parameter_decl rule: {:?}", p.as_rule())
    }
  }

  return fields;
}

fn consume_struct_decl(pair: Pair<Rule>) -> Vec<VariableDefinition> {
  let mut fields: Vec<VariableDefinition> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::fields_decl => {
        fields = consume_fields_decl(p);
      }
      _ => println!("unreachable struct rule: {:?}", p.as_rule())
    };
  }
  return fields;
}

fn consume_fields_decl(pair: Pair<Rule>) -> Vec<VariableDefinition> {
  let mut fields: Vec<VariableDefinition> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::name_type_def => {
        fields.push(consume_parameter(p));
      }
      _ => println!("unreachable fields rule: {:?}", p.as_rule())
    };
  }
  return fields;
}

fn consume_parameter(pair: Pair<Rule>) -> VariableDefinition {
  let mut field = VariableDefinition::default();
  field.loc = Loc::from_pair(pair.as_span());
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        field.name = p.as_str().to_string();
      }
      Rule::param_type => {
        field.type_type = p.as_str().to_string();
      }
      Rule::value => {
        field.initializer = Some(p.as_str().to_string());
      }
      _ => println!("unreachable parameter rule: {:?}", p.as_rule())
    };
  }
  return field;
}

fn consume_value_object(pair: Pair<Rule>) -> ValueObjectDecl {
  let mut value_object = ValueObjectDecl::default();
  value_object.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        value_object.name = p.as_str().to_string();
      }
      Rule::constructor_decl => {
        value_object.fields = consume_constructor_decl(p);
      }
      Rule::struct_decl => {
        value_object.fields = consume_struct_decl(p);
      }
      _ => println!("unreachable value_object rule: {:?}", p.as_rule())
    };
  }
  return value_object;
}

fn consume_component(pair: Pair<Rule>) -> ComponentDecl {
  let mut component = ComponentDecl::default();
  component.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        component.name = p.as_str().to_string();
      }
      Rule::inline_doc => {
        component.inline_doc = parse_inline_doc(p);
      }
      Rule::attr_decl => {
        component.attributes.push(consume_attribute(p));
      }
      Rule::used_domain_objects_decl => {
        component.used_domain_objects = [component.used_domain_objects, consume_use_domain_object(p)].concat();
      }
      _ => println!("unreachable component rule: {:?}", p.as_rule())
    };
  }
  return component;
}

fn consume_attribute(pair: Pair<Rule>) -> AttributeDefinition {
  let mut attribute = AttributeDefinition::default();
  attribute.loc = Loc::from_pair(pair.as_span());
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        attribute.key = p.as_str().to_string();
      }
      Rule::attr_value => {
        attribute.value = consume_attr_value(p);
      }
      Rule::attr_list => {
        for inner in p.into_inner() {
          match inner.as_rule() {
            Rule::attr_value => {
              attribute.value = [attribute.value, consume_attr_value(inner)].concat();
            }
            _ => println!("unreachable attr_list rule: {:?}", inner.as_rule())
          };
        }
      }
      _ => println!("unreachable attribute rule: {:?}", p.as_rule())
    };
  }
  return attribute;
}

fn consume_attr_value(pair: Pair<Rule>) -> Vec<String> {
  let mut values = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        values.push(p.as_str().to_string());
      }
      Rule::string => {
        values.push(parse_string(p.as_str()));
      }
      _ => println!("unreachable attr_value rule: {:?}", p.as_rule())
    };
  }
  return values;
}

fn consume_implementation(pair: Pair<Rule>) -> ImplementationDecl {
  let mut implementation = ImplementationDecl::default();
  implementation.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        implementation.name = p.as_str().to_string();
      }
      Rule::inline_doc => {
        implementation.inline_doc = parse_inline_doc(p);
      }
      Rule::endpoint_decl => {
        implementation.endpoint = consume_endpoint(p);
      }
      Rule::flow_decl => {
        implementation.flow = consume_flow(p);
      }
      Rule::set_target_object => {
        implementation.target = Some(consume_set_target_object(p));
      }
      _ => println!("unreachable implementation rule: {:?}", p.as_rule())
    };
  }
  return implementation;
}

fn consume_set_target_object(pair: Pair<Rule>) -> ImplementationTarget {
  let mut target = ImplementationTarget::default();
  target.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::set_aggregate_name => {
        target.target_type = ImplementationTargetType::Aggregate;
        target.name = p.as_str().to_string();
      }
      Rule::set_entity_name => {
        target.target_type = ImplementationTargetType::Entity;
        target.name = p.as_str().to_string();
      }
      _ => println!("unreachable set_target_object rule: {:?}", p.as_rule())
    };
  }
  return target;
}

fn consume_endpoint(pair: Pair<Rule>) -> EndpointDecl {
  let mut endpoint = EndpointDecl::default();
  endpoint.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        endpoint.name = p.as_str().to_string();
      }
      Rule::http_request_decl => {
        for inner in p.into_inner() {
          match inner.as_rule() {
            Rule::http_method => {
              endpoint.method = inner.as_str().to_string();
            }
            Rule::uri => {
              endpoint.uri = parse_string(inner.as_str());
            }
            _ => println!("unreachable http_request_decl rule: {:?}", inner.as_rule())
          }
        }
      }
      Rule::request_body => {
        for inner in p.into_inner() {
          match inner.as_rule() {
            Rule::identifier => {
              let loc = Loc::from_pair(inner.as_span());
              endpoint.request = Some(HttpRequestDecl {
                name: inner.as_str().to_string(),
                loc,
              });
            }
            _ => println!("unreachable http_request_decl rule: {:?}", inner.as_rule())
          }
        }
      }
      Rule::authorization_decl => {
        endpoint.authorization = Some(consume_authorization(p));
      }
      Rule::http_response_decl => {
        endpoint.response = Some(consume_http_response(p));
      }
      _ => println!("unreachable endpoint rule: {:?}", p.as_rule())
    };
  }
  return endpoint;
}

fn consume_struct(pair: Pair<Rule>) -> StructDecl {
  let mut struct_decl = StructDecl::default();
  struct_decl.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        struct_decl.name = p.as_str().to_string();
      }
      Rule::inline_doc => {
        struct_decl.inline_doc = parse_inline_doc(p);
      }
      Rule::fields_decl => {
        struct_decl.fields = consume_fields_decl(p);
      }
      _ => println!("unreachable struct rule: {:?}", p.as_rule())
    };
  }
  return struct_decl;
}

fn consume_authorization(pair: Pair<Rule>) -> AuthorizationDecl {
  let mut authorization = AuthorizationDecl::default();
  authorization.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::authorization_type => {
        authorization.auth_type = p.as_str().to_string();
      }
      Rule::username => {
        authorization.username = Some(p.as_str().to_string());
      }
      Rule::password => {
        authorization.password = Some(p.as_str().to_string());
      }
      _ => println!("unreachable authorization rule: {:?}", p.as_rule())
    };
  }
  return authorization;
}

fn consume_http_response(pair: Pair<Rule>) -> HttpResponseDecl {
  let mut response = HttpResponseDecl::default();
  response.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        response.name = p.as_str().to_string();
      }
      _ => println!("unreachable http_response rule: {:?}", p.as_rule())
    };
  }
  return response;
}

fn consume_flow(pair: Pair<Rule>) -> Option<FlowDecl> {
  let mut flow = FlowDecl::default();
  flow.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::inline_doc => {
        flow.inline_doc = parse_inline_doc(p);
      }
      Rule::via_method_decl => {
        flow.steps.push(StepDecl::MethodCall(consume_via_method_decl(p)));
      }
      Rule::via_message_decl => {
        flow.steps.push(StepDecl::Message(consume_via_message_decl(p)));
      }
      _ => println!("unreachable flow rule: {:?}", p.as_rule())
    };
  }
  if flow.steps.len() == 0 {
    return None;
  }

  return Some(flow);
}

fn consume_via_method_decl(pair: Pair<Rule>) -> MethodCallDecl {
  let mut method_call = MethodCallDecl::default();
  method_call.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        method_call.name = p.as_str().to_string();
      }
      Rule::object_name => {
        method_call.object = p.as_str().to_string();
      }
      Rule::method_name => {
        method_call.method = p.as_str().to_string();
      }
      Rule::parameters_decl => {
        method_call.arguments = consume_parameters(p);
      }
      Rule::receive_object => {
        method_call.return_type = Some(consume_parameter(p.into_inner().next().unwrap()));
      }
      _ => println!("unreachable via_method_decl rule: {:?}", p.as_rule())
    };
  }
  return method_call;
}

fn consume_via_message_decl(pair: Pair<Rule>) -> MessageDecl {
  let mut message = MessageDecl::default();
  message.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::object_name => {
        message.from = p.as_str().to_string();
      }
      Rule::topic_name => {
        message.topic = p.as_str().to_string();
      }
      Rule::pass_object => {
        message.message = p.as_str().to_string();
      }
      _ => println!("unreachable via_message_decl rule: {:?}", p.as_rule())
    };
  }
  return message;
}

fn consume_layered(pair: Pair<Rule>) -> LayeredDecl {
  let mut layered = LayeredDecl::default();
  layered.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        layered.name = p.as_str().to_string();
      }
      Rule::inline_doc => {
        layered.inline_doc = parse_inline_doc(p);
      }
      Rule::dependency_decl => {
        layered.dependencies = consume_dependency_decl(p);
      }
      Rule::layer_decl => {
        layered.layers.push(consume_layer_decl(p));
      }
      _ => println!("unreachable layered rule: {:?}", p.as_rule())
    };
  }

  return layered;
}

fn consume_layer_decl(pair: Pair<Rule>) -> LayerDecl {
  let mut layer = LayerDecl::default();
  layer.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        layer.name = p.as_str().to_string();
      }
      Rule::inline_doc => {
        layer.inline_doc = parse_inline_doc(p);
      }
      Rule::package_def => {
        for p in p.into_inner() {
          match p.as_rule() {
            Rule::package => {
              layer.package = parse_string(p.as_str());
            }
            _ => println!("unreachable package_def rule: {:?}", p.as_rule())
          };
        }
      }
      _ => println!("unreachable layer rule: {:?}", p.as_rule())
    };
  }

  return layer;
}

fn consume_dependency_decl(pair: Pair<Rule>) -> Vec<LayerRelationDecl> {
  let mut relations: Vec<LayerRelationDecl> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::dependency_entry => {
        relations.push(consume_dependency_entry(p));
      }

      _ => println!("unreachable dependency rule: {:?}", p.as_rule())
    };
  }
  return relations;
}

fn consume_dependency_entry(pair: Pair<Rule>) -> LayerRelationDecl {
  let mut relation = LayerRelationDecl::default();
  relation.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::source => {
        relation.source = parse_ident_or_string(p);
      }
      Rule::target => {
        relation.target = parse_ident_or_string(p);
      }
      Rule::rs_left_to_right => {}
      _ => println!("unreachable dependency entry: {:?}", p.as_rule())
    };
  }

  relation
}

fn consume_source_sets(pair: Pair<Rule>) -> SourceSetsDecl {
  let mut source_sets = SourceSetsDecl::default();
  source_sets.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        source_sets.name = p.as_str().to_string();
      }
      Rule::source_set_decl => {
        source_sets.source_sets.push(consume_source_set_decl(p));
      }
      _ => println!("unreachable source_sets rule: {:?}", p.as_rule())
    };
  }

  source_sets
}

fn consume_source_set_decl(pair: Pair<Rule>) -> SourceSetDecl {
  let mut source_set = SourceSetDecl::default();
  source_set.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        source_set.name = p.as_str().to_string();
      }
      Rule::attr_decl => {
        source_set.attributes.push(consume_attribute(p));
      }
      _ => println!("unreachable source_set rule: {:?}", p.as_rule())
    };
  }

  return source_set;
}

fn consume_env(pair: Pair<Rule>) -> EnvDecl {
  let mut env = EnvDecl::default();
  env.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        env.name = p.as_str().to_string();
      }
      Rule::datasource_decl => {
        env.datasource = Some(consume_datasource_decl(p));
      }
      Rule::server_decl => {
        env.server = Some(consume_server_decl(p));
      }
      Rule::custom_decl => {
        env.customs.push(consume_custom_decl(p));
      }
      _ => println!("unreachable env rule: {:?}", p.as_rule())
    };
  }

  env
}

fn consume_datasource_decl(pair: Pair<Rule>) -> DatasourceDecl {
  let mut attrs: HashMap<String, String> = HashMap::default();
  let loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::attr_decl => {
        let attr = consume_attribute(p);
        attrs.insert(attr.key.clone(), attr.value[0].clone());
      }
      _ => println!("unreachable datasource rule: {:?}", p.as_rule())
    };
  }

  let mut decl = DatasourceDecl::default();
  decl.loc = loc;
  decl.url = attrs.get("url").unwrap_or(&"".to_string()).clone();
  decl.driver = attrs.get("driver").unwrap_or(&"".to_string()).clone();
  decl.port = attrs.get("port").unwrap_or(&"".to_string()).clone();
  decl.host = attrs.get("host").unwrap_or(&"".to_string()).clone();
  decl.database = attrs.get("database").unwrap_or(&"".to_string()).clone();
  decl.username = attrs.get("username").unwrap_or(&"".to_string()).clone();
  decl.password = attrs.get("password").unwrap_or(&"".to_string()).clone();

  decl
}

fn consume_server_decl(pair: Pair<Rule>) -> ServerDecl {
  let mut attrs: HashMap<String, String> = HashMap::default();
  let loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::attr_decl => {
        let attr = consume_attribute(p);
        attrs.insert(attr.key.clone(), attr.value[0].clone());
      }
      _ => println!("unreachable server rule: {:?}", p.as_rule())
    };
  }

  let mut decl = ServerDecl::default();
  decl.loc = loc;
  decl.port = attrs.get("port")
    .unwrap_or(&default_config::SERVER_PORT.to_string())
    .parse()
    .unwrap_or(default_config::SERVER_PORT)
    .clone();
  decl
}

fn consume_custom_decl(pair: Pair<Rule>) -> CustomDecl {
  let mut decl = CustomDecl::default();
  decl.loc = Loc::from_pair(pair.as_span());

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        decl.name = p.as_str().to_string();
      }
      Rule::attr_decl => {
        decl.attributes.push(consume_attribute(p));
      }
      _ => println!("unreachable server rule: {:?}", p.as_rule())
    };
  }

  decl
}

fn parse_ident_or_string(pair: Pair<Rule>) -> String {
  let mut ident = String::new();
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        ident = p.as_str().to_string();
      }
      Rule::string => {
        ident = parse_string(p.as_str());
      }
      _ => println!("unreachable ident_or_string rule: {:?}", p.as_rule())
    };
  }

  ident
}

fn parse_string(str: &str) -> String {
  let mut s = str.to_string();
  s.remove(0);
  s.remove(s.len() - 1);

  s
}

fn parse_inline_doc(pair: Pair<Rule>) -> String {
  let len = "\"\"\"".len();

  pair.as_str().chars().skip(len).take(pair.as_str().len() - len * 2).collect()
}

#[cfg(test)]
mod tests {
  use crate::parser::ast::*;
  use crate::parser::ast::ImplementationTargetType::Aggregate;
  use crate::parser::ast::RelationDirection::{BiDirected, PositiveDirected};
  use crate::parser::ast::StepDecl::{Message, MethodCall};
  use crate::parser::parser::parse;

  #[test]
  fn parse_context_map() {
    let decls = parse(r#"
ContextMap {
  ShoppingCarContext -> MallContext;
  ShoppingCarContext <-> MallContext;
}

Context ShoppingCarContext {

}
"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::ContextMap(ContextMapDecl {
      name: Identifier {
        name: "".to_string(),
        loc: Default::default(),
      },
      contexts: vec![
        BoundedContextDecl {
          name: "MallContext".to_string(),
          domain_events: vec![],
          aggregates: vec![],
          used_domain_objects: vec![],
          loc: Loc(76, 87),
        },
        BoundedContextDecl {
          name: "ShoppingCarContext".to_string(),
          domain_events: vec![],
          aggregates: vec![],
          used_domain_objects: vec![],
          loc: Loc(53, 71),
        },
      ],
      relations: vec![
        ContextRelation { source: "ShoppingCarContext".to_string(), target: "MallContext".to_string(), direction: PositiveDirected, source_types: vec![], target_types: vec![] },
        ContextRelation { source: "ShoppingCarContext".to_string(), target: "MallContext".to_string(), direction: BiDirected, source_types: vec![], target_types: vec![] },
      ],
      loc: Loc(1, 90),
    }));
  }

  #[test]
  fn long_string() {
    let decls = parse(r#"
Aggregate Sample {
  """ inline doc sample
just for test
"""
}
"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::Aggregate(AggregateDecl {
      name: "Sample".to_string(),
      inline_doc: r#" inline doc sample
just for test
"#.to_string(),
      used_domain_objects: vec![],
      entities: vec![],
      value_objects: vec![],
      domain_events: vec![],
      loc: Loc(1, 63),
    }));
  }

  #[test]
  fn aggregate() {
    let decls = parse(r#"
Aggregate ShoppingCart {
  Entity Product {
    constructor(name: String, price: Money)
  }
}
"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::Aggregate(AggregateDecl {
      name: "ShoppingCart".to_string(),
      inline_doc: "".to_string(),
      used_domain_objects: vec![],
      entities: vec![EntityDecl {
        is_aggregate_root: false,
        name: "Product".to_string(),
        identify: Default::default(),
        inline_doc: "".to_string(),
        fields: vec![
          VariableDefinition {
            name: "name".to_string(),
            type_type: "String".to_string(),
            initializer: None,
            loc: Loc(61, 73),
          },
          VariableDefinition {
            name: "price".to_string(),
            type_type: "Money".to_string(),
            initializer: None,
            loc: Loc(75, 87),
          }],
        value_objects: vec![],
        loc: Loc(28, 92),
      }],
      value_objects: vec![],
      domain_events: vec![],
      loc: Loc(1, 94),
    }))
  }

  #[test]
  fn full_sample() {
    parse("
ContextMap {
  SalesContext <-> SalesContext;
}

Context SalesContext {
  Aggregate SalesOrder {
    Entity SalesOrderLine {
      constructor(product: Product, quantity: Quantity)
    }
  }
}

Entity Opportunity {
  constructor(
    id: String,
    name: String,
    description: String,
    status: OpportunityStatus,
    amount: Money,
    probability: Probability,
    closeDate: Date,
    contacts: Vec<Contact>,
    products: Vec<Product>,
    notes: Vec<Note>,
    attachments: Vec<Attachment>,
    activities: Vec<Activity>,
    tasks: Vec<Task>,
    events: Vec<Event>,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String
  )
}

Entity Pipeline {
  constructor(
    id: String,
    name: String,
    description: String,
    stages: Vec<Stage>,
    opportunities: Vec<Opportunity>,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String
  )
}

Entity Contact {
  constructor(
    id: String,
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    title: String,
    department: String,
    account: Account,
    address: Address,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String,
  )
}

Entity Account {
  constructor(
    id: String,
    name: String,
    website: String,
    phone: String,
    industry: String,
    employees: String,
    annualRevenue: Money,
    billingAddress: Address,
    shippingAddress: Address,
    contacts: Vec<Contact>,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String,
  )
}

Entity Product {
  constructor(
    id: String,
    name: String,
    description: String,
    price: Money,
    category: String,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String,
  )
}

Entity Territory {
  constructor(
    id: String,
    name: String,
    description: String,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String,
  )
}

Entity SalesPerson {
  constructor(
    id: String,
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    title: String,
    department: String,
    account: Account,
    address: Address,
    territories: Vec<Territory>,
    created: DateTime,
    createdBy: String,
    modified: DateTime,
    modifiedBy: String,
  )
}
").unwrap();
  }

  #[test]
  fn basic_vo_inline_aggregate() {
    let decls = parse(r#"Context Cart {
  Aggregate Cart {
    Entity Cart {
      ValueObject CartId
      ValueObject CartStatus
      ValueObject CartItem
      ValueObject CartItemQuantity
      ValueObject CartItemPrice
      ValueObject CartItemTotal
      ValueObject CartTotal
    }
  }
}"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::BoundedContext(BoundedContextDecl {
      name: "Cart".to_string(),
      domain_events: vec![],
      aggregates: vec![
        AggregateDecl {
          name: "Cart".to_string(),
          inline_doc: "".to_string(),
          used_domain_objects: vec![],
          entities: vec![EntityDecl {
            is_aggregate_root: false,
            name: "Cart".to_string(),
            identify: Default::default(),
            inline_doc: "".to_string(),
            fields: vec![],
            value_objects: vec![
              ValueObjectDecl {
                name: "CartId".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(58, 83),
              },
              ValueObjectDecl {
                name: "CartStatus".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(83, 112),
              },
              ValueObjectDecl {
                name: "CartItem".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(112, 139),
              },
              ValueObjectDecl {
                name: "CartItemQuantity".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(139, 174),
              },
              ValueObjectDecl {
                name: "CartItemPrice".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(174, 206),
              },
              ValueObjectDecl {
                name: "CartItemTotal".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(206, 238),
              },
              ValueObjectDecl {
                name: "CartTotal".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
                loc: Loc(238, 264),
              },
            ],
            loc: Loc(38, 265),
          }],
          value_objects: vec![],
          domain_events: vec![],
          loc: Loc(17, 269),
        }
      ],
      used_domain_objects: vec![],
      loc: Loc(0, 271),
    }));
  }

  #[test]
  fn bind_api() {
    let decls = parse(r#"
Component SalesComponent {
  name = 'Sample Phodal';
  type: Application;
  Aggregate SalesOrder;
}
"#);

    assert_eq!(decls.unwrap()[0], FklDeclaration::Component(ComponentDecl {
      name: "SalesComponent".to_string(),
      inline_doc: "".to_string(),
      component_type: ComponentType::Application,
      attributes: vec![
        AttributeDefinition {
          key: "name".to_string(),
          value: vec!["Sample Phodal".to_string()],
          loc: Loc(30, 53),
        },
        AttributeDefinition {
          key: "type".to_string(),
          value: vec!["Application".to_string()],
          loc: Loc(56, 74),
        },
      ],
      used_domain_objects: vec![
        UsedDomainObject { name: "SalesOrder".to_string(), loc: Loc(87, 97)  },
      ],
      loc: Loc(1, 100),
    }));
  }

  #[test]
  #[ignore]
  fn rel_with_context_map() {
    let decls = parse(r#"
ContextMap Mall {
  SalesContext [ OHS ] <-> OrderContext [ rel = "ACL, OHS" ];
}
"#).unwrap();

    let except = FklDeclaration::ContextMap(ContextMapDecl {
      name: Identifier {
        name: "Mall".to_string(),
        loc: Loc(11, 15),
      },
      contexts: vec![
        BoundedContextDecl { name: "OrderContext".to_string(), domain_events: vec![], aggregates: vec![], used_domain_objects: vec![], loc: Loc(65, 77) },
        BoundedContextDecl { name: "SalesContext".to_string(), domain_events: vec![], aggregates: vec![], used_domain_objects: vec![], loc: Loc(20, 32)  },
      ],
      relations: vec![ContextRelation {
        source: "SalesContext".to_string(),
        target: "OrderContext".to_string(),
        direction: BiDirected,
        source_types: vec!["OHS".to_string()],
        target_types: vec!["ACL".to_string(), "OHS".to_string()],
      }],
      loc: Loc(1, 89),
    });
    assert_eq!(decls[0], except);

    let order2 = parse(r#"ContextMap Mall {
  SalesContext [ OHS ] <-> [rel = "ACL, OHS" ] OrderContext;
}"#).unwrap();
    assert_eq!(order2[0], except);
  }

  #[test]
  fn rel_with_context_map_with_inline_doc() {
    let decls = parse(r#"Entity Reservation  {
  Struct {
    id: String;
    token: UUID;
    status: ReservationStatus = ReservationStatus.OPEN;
    expiresAt: LocalDateTime;
    createdAt: LocalDateTime;
    screeningId: String;
    screeningStartTime: LocalDateTime;
    name: String;
    surname: String;
    tickets: Set<Ticket>;
    totalPrice: BigDecimal;
  }
}"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::Entity(EntityDecl {
      is_aggregate_root: false,
      name: "Reservation".to_string(),
      identify: VariableDefinition {
        name: "".to_string(),
        type_type: "".to_string(),
        initializer: None,
        loc: Default::default(),
      },
      inline_doc: "".to_string(),
      fields: vec![
        VariableDefinition { name: "id".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(37, 47) },
        VariableDefinition { name: "token".to_string(), type_type: "UUID".to_string(), initializer: None, loc: Loc(53, 64) },
        VariableDefinition { name: "status".to_string(), type_type: "ReservationStatus".to_string(), initializer: Some("ReservationStatus.OPEN".to_string()), loc: Loc(70, 120) },
        VariableDefinition { name: "expiresAt".to_string(), type_type: "LocalDateTime".to_string(), initializer: None, loc: Loc(126, 150) },
        VariableDefinition { name: "createdAt".to_string(), type_type: "LocalDateTime".to_string(), initializer: None, loc: Loc(156, 180) },
        VariableDefinition { name: "screeningId".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(186, 205) },
        VariableDefinition { name: "screeningStartTime".to_string(), type_type: "LocalDateTime".to_string(), initializer: None, loc: Loc(211, 244) },
        VariableDefinition { name: "name".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(250, 262) },
        VariableDefinition { name: "surname".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(268, 283) },
        VariableDefinition { name: "tickets".to_string(), type_type: "Set<Ticket>".to_string(), initializer: None, loc: Loc(289, 309) },
        VariableDefinition { name: "totalPrice".to_string(), type_type: "BigDecimal".to_string(), initializer: None, loc: Loc(315, 337) }],
      value_objects: vec![],
      loc: Loc(0, 344),
    }));
  }

  #[test]
  fn use_vo() {
    let decls = parse(r#"Context Cinema {
  Aggregate Cinema;
}

Aggregate Cinema {
  Entity Cinema, ScreeningRoom, Seat;
}
"#).unwrap();

    assert_eq!(decls[1], FklDeclaration::Aggregate(
      AggregateDecl {
        name: "Cinema".to_string(),
        inline_doc: "".to_string(),
        used_domain_objects: vec![
          UsedDomainObject { name: "Cinema".to_string(), loc: Loc(68, 74) },
          UsedDomainObject { name: "ScreeningRoom".to_string(), loc: Loc(76, 89) },
          UsedDomainObject { name: "Seat".to_string(), loc: Loc(91, 95) }],
        entities: vec![],
        value_objects: vec![],
        domain_events: vec![],
        loc: Loc(40, 98),
      })
    );
  }

  #[test]
  fn aggregate_binding_syntax() {
    let result = parse(r#"
impl CinemaCreatedEvent {
  endpoint {
    GET "/book/{id}";
    authorization: Basic admin admin;
    response: Cinema;
  }
}

struct Cinema {
  id: String;
  name: String;
  address: String;
  rooms: Set<ScreeningRoom>;
}
"#).unwrap();

    assert_eq!(result[0], FklDeclaration::Implementation(ImplementationDecl {
      name: "CinemaCreatedEvent".to_string(),
      inline_doc: "".to_string(),
      qualified_name: "".to_string(),
      endpoint: EndpointDecl {
        name: "".to_string(),
        method: "GET".to_string(),
        uri: "/book/{id}".to_string(),
        authorization: Some(AuthorizationDecl {
          auth_type: "Basic".to_string(),
          username: Some("admin".to_string()),
          password: Some("admin".to_string()),
          loc: Loc(66, 99),
        }),
        request: None,
        response: Some(HttpResponseDecl {
          name: "Cinema".to_string(),
          loc: Loc(104, 121),
        }),
        loc: Loc(29, 125),
      },
      target: None,
      flow: None,
      loc: Loc(1, 127),
    }));

    assert_eq!(result[1], FklDeclaration::Struct(StructDecl {
      name: "Cinema".to_string(),
      inline_doc: "".to_string(),
      fields: vec![
        VariableDefinition { name: "id".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(147, 157) },
        VariableDefinition { name: "name".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(161, 173) },
        VariableDefinition { name: "address".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(177, 192) },
        VariableDefinition { name: "rooms".to_string(), type_type: "Set<ScreeningRoom>".to_string(), initializer: None, loc: Loc(196, 221) },
      ],
      loc: Loc(129, 224)
    }));
  }

  #[test]
  fn error_handling() {
    let result = parse(r#"
imple CinemaCreatedEvent {

}"#);
    match result {
      Err(e) => {
        let string = format!("{}", e);
        assert_eq!(string, r#" --> 2:1
  |
2 | imple CinemaCreatedEvent {
  | ^---
  |
  = expected EOI or declaration"#);
      }
      _ => assert!(false),
    };
  }

  #[test]
  fn impl_with_flow() {
    let decls = parse(r#"impl CinemaUpdated {
    aggregate: Cinema;
    endpoint {
        POST "/book/{id}";
        request: CinemaUpdatedRequest;
        authorization: Basic admin admin;
        response: Cinema;
    }

    flow {
        via UserRepository::getUserById receive user: User
        via UserRepository::save(user: User) receive user: User;
        via MessageQueue send CinemaCreated to "CinemaCreated"
    }
}
"#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();

    assert_eq!(decls[0], FklDeclaration::Implementation(ImplementationDecl {
      name: "CinemaUpdated".to_string(),
      inline_doc: "".to_string(),
      qualified_name: "".to_string(),
      endpoint: EndpointDecl {
        name: "".to_string(),
        method: "POST".to_string(),
        uri: "/book/{id}".to_string(),
        authorization: Some(AuthorizationDecl {
          auth_type: "Basic".to_string(),
          username: Some("admin".to_string()),
          password: Some("admin".to_string()),
          loc: Loc(133, 166),
        }),
        request: Some(HttpRequestDecl {
          name: "CinemaUpdatedRequest".to_string(),
          loc: Loc(103, 123),
        }),
        response: Some(HttpResponseDecl {
          name: "Cinema".to_string(),
          loc: Loc(175, 192),
        }),
        loc: Loc(48, 198),
      },
      target: Some(ImplementationTarget {
        target_type: Aggregate,
        name: "Cinema".to_string(),
        loc: Loc(25, 43),
      }),
      flow: Some(FlowDecl
      {
        inline_doc: "".to_string(),
        steps: vec![
          MethodCall(MethodCallDecl {
            name: "".to_string(),
            object: "UserRepository".to_string(),
            method: "getUserById".to_string(),
            arguments: vec![],
            return_type: Some(VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
              loc: Loc(259, 278),
            }),
            loc: Loc(219, 278),
          }),
          MethodCall(MethodCallDecl {
            name: "".to_string(),
            object: "UserRepository".to_string(),
            method: "save".to_string(),
            arguments: vec![VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
              loc: Loc(303, 313),
            }],
            return_type: Some(VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
              loc: Loc(323, 333),
            }),
            loc: Loc(278, 334),
          }),
          Message(MessageDecl {
            from: "MessageQueue".to_string(),
            topic: "\"CinemaCreated\"".to_string(),
            message: "CinemaCreated".to_string(),
            loc: Loc(343, 402),
          }),
        ],
        loc: Loc(204, 403),
      }),
      loc: Loc(0, 405),
    }));
  }

  #[test]
  fn layered_architecture() {
    let decls = parse(r#"layered DDD {
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
}"#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();

    assert_eq!(decls[0], FklDeclaration::Layered(LayeredDecl {
      name: "DDD".to_string(),
      inline_doc: "".to_string(),
      dependencies: vec![
        LayerRelationDecl {
          source: "rest".to_string(),
          target: "application".to_string(),
          loc: Loc(33, 56),
        },
        LayerRelationDecl {
          source: "rest".to_string(),
          target: "domain".to_string(),
          loc: Loc(61, 79),
        },
        LayerRelationDecl {
          source: "domain".to_string(),
          target: "application".to_string(),
          loc: Loc(84, 109),
        },
        LayerRelationDecl {
          source: "application".to_string(),
          target: "infrastructure".to_string(),
          loc: Loc(114, 147),
        },
        LayerRelationDecl {
          source: "rest".to_string(),
          target: "infrastructure".to_string(),
          loc: Loc(152, 178),
        },
      ],
      layers: vec![
        LayerDecl {
          name: "rest".to_string(),
          inline_doc: "".to_string(),
          package: "com.example.book".to_string(),
          loc: Loc(185, 235),
        },
        LayerDecl {
          name: "domain".to_string(),
          inline_doc: "".to_string(),
          package: "com.example.domain".to_string(),
          loc: Loc(238, 292),
        },
        LayerDecl {
          name: "application".to_string(),
          inline_doc: "".to_string(),
          package: "com.example.application".to_string(),
          loc: Loc(295, 358),
        },
        LayerDecl {
          name: "infrastructure".to_string(),
          inline_doc: "".to_string(),
          package: "com.example.infrastructure".to_string(),
          loc: Loc(361, 430),
        },
      ],
      loc: Loc(0, 432),
    }));
  }

  #[test]
  fn parse_source_set() {
    let decls = parse(r#"SourceSet sourceSet {
  feakin {
    srcDir: ["src/main/resources/uml"]
  }
  puml {
    parser: "PlantUML"
    srcDir: ["src/main/resources/uml"]
  }
}"#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();

    assert_eq!(decls[0], FklDeclaration::SourceSets(SourceSetsDecl {
      name: "sourceSet".to_string(),
      source_sets: vec![
        SourceSetDecl {
          name: "feakin".to_string(),
          attributes: vec![
            AttributeDefinition {
              key: "srcDir".to_string(),
              value: vec!["src/main/resources/uml".to_string()],
              loc: Loc(37, 74),
            }],
          loc: Loc(24, 75),
        },
        SourceSetDecl {
          name: "puml".to_string(),
          attributes: vec![
            AttributeDefinition {
              key: "parser".to_string(),
              value: vec!["PlantUML".to_string()],
              loc: Loc(89, 112),
            },
            AttributeDefinition {
              key: "srcDir".to_string(),
              value: vec!["src/main/resources/uml".to_string()],
              loc: Loc(112, 149),
            }],
          loc: Loc(78, 150),
        }],
      loc: Loc(0, 152),
    }));
  }

  #[test]
  fn aggregate_domain_event() {
    let decls = parse(r#"Aggregate User {
  DomainEvent UserCreated, UserUpdated;
}"#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();

    assert_eq!(decls[0], FklDeclaration::Aggregate(AggregateDecl {
      name: "User".to_string(),
      inline_doc: "".to_string(),
      used_domain_objects: vec![],
      entities: vec![],
      value_objects: vec![],
      domain_events: vec![
        DomainEventDecl { name: "UserCreated".to_string(), loc: Loc(31, 42) },
        DomainEventDecl { name: "UserUpdated".to_string(), loc: Loc(44, 55) },
      ],
      loc: Loc(0, 58),
    }));
  }

  #[test]
  fn env_database() {
    let decls = parse(r#"
env Local {
  datasource {
    url: "jdbc:postgresql://localhost:5432/yourdb"
    driver: "org.postgresql.Driver"
    username: "youruser"
    password: "yourpassword"
  }
}"#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();

    assert_eq!(decls[0], FklDeclaration::Env(EnvDecl {
      name: "Local".to_string(),
      inline_doc: "".to_string(),
      datasource: Some(DatasourceDecl {
        url: "jdbc:postgresql://localhost:5432/yourdb".to_string(),
        host: "".to_string(),
        port: "".to_string(),
        driver: "org.postgresql.Driver".to_string(),
        username: "youruser".to_string(),
        password: "yourpassword".to_string(),
        database: "".to_string(),
        loc: Loc(15, 172)
      }),
      server: None,
      customs: vec![],
      loc: Loc(1, 174)
    }));
  }

  #[test]
  fn env_server() {
    let decls = parse(r#"
env Local {
  server {
    port: 8899
  }
}"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::Env(EnvDecl {
      name: "Local".to_string(),
      inline_doc: "".to_string(),
      datasource: None,
      server: Some(ServerDecl {
        port: 8899,
        attributes: vec![],
        loc: Loc(15, 42)
      }),
      customs: vec![],
      loc: Loc(1, 44)
    }));
  }

  #[test]
  fn custom_env() {
    let decls = parse(r#"
env Local {
  kafka {
    host: "localhost"
    port: 9092
  }
}"#).unwrap();

    assert_eq!(decls[0], FklDeclaration::Env(EnvDecl {
      name: "Local".to_string(),
      inline_doc: "".to_string(),
      datasource: None,
      server: None,
      customs: vec![
        CustomDecl {
          name: "kafka".to_string(),
          inline_doc: "".to_string(),
          attributes: vec![
            AttributeDefinition {
              key: "host".to_string(),
              value: vec!["localhost".to_string()],
              loc: Loc(27, 49),
            },
            AttributeDefinition {
              key: "port".to_string(),
              value: vec!["9092".to_string()],
              loc: Loc(49, 62),
            }],
          loc: Loc(15, 63)
        }
      ],
      loc: Loc(1, 65)
    }));
  }

  #[test]
  fn syntax_sugar() {
    let decls = parse(r#"ContextMap architecture {
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
}
"#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();

    assert_eq!(decls[0], FklDeclaration::ContextMap(ContextMapDecl {
      name: Identifier {
        name: "architecture".to_string(),
        loc: Loc(11, 23),
      },
      contexts: vec![
        BoundedContextDecl {
          name: "analyze".to_string(),
          aggregates: vec![
            AggregateDecl {
              name: "ArchSystem".to_string(),
              inline_doc: "".to_string(),
              used_domain_objects: vec![],
              entities: vec![
                EntityDecl {
                  name: "ArchSystem".to_string(),
                  is_aggregate_root: false,
                  identify: Default::default(),
                  inline_doc: "".to_string(),
                  fields: vec![
                    VariableDefinition { name: "id".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(116, 126) },
                    VariableDefinition { name: "name".to_string(), type_type: "String".to_string(), initializer: None, loc: Loc(144, 156) },
                  ],
                  value_objects: vec![],
                  loc: Loc(91, 171),
                },
                EntityDecl {
                  name: "ArchComponent".to_string(),
                  is_aggregate_root: false,
                  identify: Default::default(),
                  inline_doc: "".to_string(),
                  fields: vec![
                    VariableDefinition {
                      name: "name".to_string(),
                      type_type: "String".to_string(),
                      initializer: None,
                      loc: Loc(253, 265),
                    },
                    VariableDefinition {
                      name: "type".to_string(),
                      type_type: "ArchComponentType".to_string(),
                      initializer: None,
                      loc: Loc(287, 327),
                    },
                  ],
                  value_objects: vec![],
                  loc: Loc(185, 342),
                },
              ],
              value_objects: vec![],
              domain_events: vec![],
              loc: Loc(56, 352),
            },
          ],
          domain_events: vec![],
          used_domain_objects: vec![],
          loc: Loc(30, 358),
        },
      ],
      relations: vec![],
      loc: Loc(0, 360),
    }));
  }

  #[test]
  fn include_other_file() {
    let _decls = parse(r#"include "./layer.rs""#).or_else(|e| {
      println!("{}", e);
      Err(e)
    }).unwrap();
  }
}
