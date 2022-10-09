// use core::panicking::panic;
use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use crate::parser::ast::{AggregateDecl, AttributeDefinition, AuthorizationDecl, BoundedContextDecl, ComponentDecl, ContextMapDecl, ContextRelation, EndpointDecl, EntityDecl, FklDeclaration, FlowDecl, HttpRequestDecl, HttpResponseDecl, Identifier, ImplementationDecl, Loc, MessageDecl, MethodCallDecl, RelationDirection, StepDecl, StructDecl, UsedDomainObject, ValueObjectDecl, VariableDefinition};
use crate::parser::parse_result::{ParseError, ParseResult};
use crate::pest::Parser;

#[derive(Parser)]
#[grammar = "parser/fkl.pest"]
pub struct FklParser;

pub fn parse(code: &str) -> ParseResult<Vec<FklDeclaration>> {
  match FklParser::parse(Rule::declarations, code) {
    Err(e) => {
      return Err(ParseError::msg(e.to_string()));
    }
    Ok(pairs) => {
      Ok(consume_declarations(pairs))
    }
  }
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
        _ => println!("unreachable content rule: {:?}", p.as_rule())
      };
    }
    return decl;
  }).collect::<Vec<FklDeclaration>>()
}

fn consume_context_map(pair: Pair<Rule>) -> ContextMapDecl {
  let mut context_decl_map: HashMap<String, BoundedContextDecl> = HashMap::new();
  let mut identify = Identifier::default();
  let mut relations: Vec<ContextRelation> = Vec::new();

  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        identify.name = p.as_str().to_string();
        identify.loc = Loc::from_pair(p.as_span());
      }
      Rule::context_node_rel => {
        let mut names: Vec<String> = vec![];
        let mut direction: RelationDirection = RelationDirection::Undirected;
        let mut source_type: Vec<String> = vec![];
        let mut target_type: Vec<String> = vec![];

        for p in p.into_inner() {
          match p.as_rule() {
            Rule::left_id | Rule::right_id => {
              let context_name = p.as_str().to_string();
              names.push(context_name.clone());
              context_decl_map.insert(context_name.clone(), BoundedContextDecl {
                name: context_name,
                domain_events: vec![],
                aggregates: vec![],
                used_domain_objects: vec![],
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
                match p.as_rule() {
                  Rule::rel_defs => {
                    source_type = rel_defs(p);
                  }
                  _ => {}
                }
              }
            }
            Rule::right_rel_defs => {
              for p in p.into_inner() {
                match p.as_rule() {
                  Rule::rel_defs => {
                    target_type = rel_defs(p);
                  }
                  _ => {}
                }
              }
            }
            _ => println!("unreachable context rel rule: {:?}", p.as_rule())
          };
        }

        relations.push(ContextRelation {
          source: names[0].clone(),
          target: names[1].clone(),
          direction,
          source_types: source_type,
          target_types: target_type,
        });
      }
      _ => println!("unreachable context_map rule: {:?}", p.as_rule())
    };
  }

  // sort context map by name
  let mut contexts = context_decl_map.into_iter().map(|(_, v)| v)
    .collect::<Vec<BoundedContextDecl>>();

  contexts.sort_by(|a, b| a.name.cmp(&b.name));

  return ContextMapDecl {
    name: identify,
    contexts,
    relations,
  };
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
      _ => println!("unreachable aggregate rule: {:?}", p.as_rule())
    };
  }

  return aggregate;
}

fn consume_entity(pair: Pair<Rule>) -> EntityDecl {
  let mut entity = EntityDecl::default();
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
        used_domain_objects.push(UsedDomainObject {
          name: p.as_str().to_string()
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
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        attribute.key = p.as_str().to_string();
      }
      Rule::attr_value => {
        attribute.value = consume_attr_value(p);
      }
      _ => println!("unreachable attribute rule: {:?}", p.as_rule())
    };
  }
  return attribute;
}

fn consume_attr_value(pair: Pair<Rule>) -> String {
  let mut value = String::new();
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::identifier => {
        value = p.as_str().to_string();
      }
      Rule::string => {
        value = parse_string(p.as_str());
      }
      _ => println!("unreachable attr_value rule: {:?}", p.as_rule())
    };
  }
  return value;
}

fn consume_implementation(pair: Pair<Rule>) -> ImplementationDecl {
  let mut implementation = ImplementationDecl::default();
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
      _ => println!("unreachable implementation rule: {:?}", p.as_rule())
    };
  }
  return implementation;
}

fn consume_endpoint(pair: Pair<Rule>) -> EndpointDecl {
  let mut endpoint = EndpointDecl::default();
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
              endpoint.request = Some(HttpRequestDecl {
                name: inner.as_str().to_string(),
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
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::authorization_type => {
        authorization.authorization_type = p.as_str().to_string();
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
    return None
  }

  return Some(flow);
}

fn consume_via_method_decl(pair: Pair<Rule>) -> MethodCallDecl {
  let mut method_call = MethodCallDecl::default();
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

fn parse_string(str: &str) -> String {
  let mut s = str.to_string();
  s.remove(0);
  s.remove(s.len() - 1);
  return s;
}

fn parse_inline_doc(pair: Pair<Rule>) -> String {
  let len = "\"\"\"".len();
  return pair.as_str().chars().skip(len).take(pair.as_str().len() - len * 2).collect();
}

#[cfg(test)]
mod tests {
  use crate::parser::ast::*;
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
        },
        BoundedContextDecl {
          name: "ShoppingCarContext".to_string(),
          domain_events: vec![],
          aggregates: vec![],
          used_domain_objects: vec![],
        },
      ],
      relations: vec![
        ContextRelation { source: "ShoppingCarContext".to_string(), target: "MallContext".to_string(), direction: PositiveDirected, source_types: vec![], target_types: vec![] },
        ContextRelation { source: "ShoppingCarContext".to_string(), target: "MallContext".to_string(), direction: BiDirected, source_types: vec![], target_types: vec![] },
      ],
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
      domain_events: vec![]
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
          },
          VariableDefinition {
            name: "price".to_string(),
            type_type: "Money".to_string(),
            initializer: None,
          }],
        value_objects: vec![],
      }],
      value_objects: vec![],
      domain_events: vec![]
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
              },
              ValueObjectDecl {
                name: "CartStatus".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
              },
              ValueObjectDecl {
                name: "CartItem".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
              },
              ValueObjectDecl {
                name: "CartItemQuantity".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
              },
              ValueObjectDecl {
                name: "CartItemPrice".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
              },
              ValueObjectDecl {
                name: "CartItemTotal".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
              },
              ValueObjectDecl {
                name: "CartTotal".to_string(),
                inline_doc: "".to_string(),
                fields: vec![],
              },
            ],
          }],
          value_objects: vec![],
          domain_events: vec![]
        }
      ],
      used_domain_objects: vec![],
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
          value: "Sample Phodal".to_string(),
        }, AttributeDefinition {
          key: "type".to_string(),
          value: "Application".to_string(),
        },
      ],
      used_domain_objects: vec![
        UsedDomainObject { name: "SalesOrder".to_string() },
      ],
    }));
  }

  #[test]
  fn rel_with_context_map() {
    let decls = parse(r#"ContextMap Mall {
  SalesContext [ OHS ] <-> OrderContext [ rel = "ACL, OHS" ];
}"#).unwrap();

    let except = FklDeclaration::ContextMap(ContextMapDecl {
      name: Identifier {
        name: "Mall".to_string(),
        loc: Loc(11, 15),
      },
      contexts: vec![
        BoundedContextDecl { name: "OrderContext".to_string(), domain_events: vec![], aggregates: vec![], used_domain_objects: vec![] },
        BoundedContextDecl { name: "SalesContext".to_string(), domain_events: vec![], aggregates: vec![], used_domain_objects: vec![] },
      ],
      relations: vec![ContextRelation {
        source: "SalesContext".to_string(),
        target: "OrderContext".to_string(),
        direction: BiDirected,
        source_types: vec!["OHS".to_string()],
        target_types: vec!["ACL".to_string(), "OHS".to_string()],
      }],
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
      },
      inline_doc: "".to_string(),
      fields: vec![
        VariableDefinition { name: "id".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "token".to_string(), type_type: "UUID".to_string(), initializer: None },
        VariableDefinition { name: "status".to_string(), type_type: "ReservationStatus".to_string(), initializer: Some("ReservationStatus.OPEN".to_string()) },
        VariableDefinition { name: "expiresAt".to_string(), type_type: "LocalDateTime".to_string(), initializer: None },
        VariableDefinition { name: "createdAt".to_string(), type_type: "LocalDateTime".to_string(), initializer: None },
        VariableDefinition { name: "screeningId".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "screeningStartTime".to_string(), type_type: "LocalDateTime".to_string(), initializer: None },
        VariableDefinition { name: "name".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "surname".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "tickets".to_string(), type_type: "Set<Ticket>".to_string(), initializer: None },
        VariableDefinition { name: "totalPrice".to_string(), type_type: "BigDecimal".to_string(), initializer: None }],
      value_objects: vec![],
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
          UsedDomainObject { name: "Cinema".to_string() },
          UsedDomainObject { name: "ScreeningRoom".to_string() },
          UsedDomainObject { name: "Seat".to_string() }],
        entities: vec![],
        value_objects: vec![],
        domain_events: vec![]
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
          authorization_type: "Basic".to_string(),
          username: Some("admin".to_string()),
          password: Some("admin".to_string()),
        }),
        request: None,
        response: Some(HttpResponseDecl {
          name: "Cinema".to_string()
        }),
      },
      flow: None,
    }));

    assert_eq!(result[1], FklDeclaration::Struct(StructDecl {
      name: "Cinema".to_string(),
      inline_doc: "".to_string(),
      fields: vec![
        VariableDefinition { name: "id".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "name".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "address".to_string(), type_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "rooms".to_string(), type_type: "Set<ScreeningRoom>".to_string(), initializer: None },
      ],
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
          authorization_type: "Basic".to_string(),
          username: Some("admin".to_string()),
          password: Some("admin".to_string()),
        }),
        request: Some(HttpRequestDecl {
          name: "CinemaUpdatedRequest".to_string()
        }),
        response: Some(HttpResponseDecl {
          name: "Cinema".to_string()
        }),
      },
      flow: Some(FlowDecl {
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
            }),
          }),
          MethodCall(MethodCallDecl {
            name: "".to_string(),
            object: "UserRepository".to_string(),
            method: "save".to_string(),
            arguments: vec![VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
            }],
            return_type: Some(VariableDefinition {
              name: "user".to_string(),
              type_type: "User".to_string(),
              initializer: None,
            }),
          }),
          Message(MessageDecl {
            from: "MessageQueue".to_string(),
            topic: "\"CinemaCreated\"".to_string(),
            message: "CinemaCreated".to_string(),
          }),
        ],
      }),
    }));
  }
}

