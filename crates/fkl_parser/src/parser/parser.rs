use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use crate::parser::ast::{AggregateDecl, AttributeDefinition, BoundedContextDecl, ComponentDecl, ContextMapDecl, ContextRelation, EntityDecl, FklDeclaration, Identifier, Loc, RelationDirection, UsedDomainObject, ValueObjectDecl, VariableDefinition};
use crate::parser::parse_result::{ParseError, ParseResult};
use crate::pest::Parser;

#[derive(Parser)]
#[grammar = "parser/fkl.pest"]
pub struct FklParser;

pub fn parse(code: &str) -> ParseResult<Vec<FklDeclaration>> {
  match FklParser::parse(Rule::declarations, code) {
    Err(e) => {
      let fancy_e = e.renamed_rules(|rule| {
        match *rule {
          _ => {
            format!("{:?}", rule)
          }
        }
      });
      return Err(ParseError::msg(fancy_e));
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
        for p in p.into_inner() {
          match p.as_rule() {
            Rule::name_type_def => {
              fields.push(consume_parameter(p));
            }
            _ => println!("unreachable parameter_decl rule: {:?}", p.as_rule())
          }
        }
      }
      _ => println!("unreachable constructor rule: {:?}", p.as_rule())
    };
  }
  return fields;
}

fn consume_struct_decl(pair: Pair<Rule>) -> Vec<VariableDefinition> {
  let mut fields: Vec<VariableDefinition> = vec![];
  for p in pair.into_inner() {
    match p.as_rule() {
      Rule::fields_decl => {
        for p in p.into_inner() {
          match p.as_rule() {
            Rule::name_type_def => {
              fields.push(consume_parameter(p));
            }
            _ => println!("unreachable struct_decl rule: {:?}", p.as_rule())
          }
        }
      }
      _ => println!("unreachable struct rule: {:?}", p.as_rule())
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
        field.field_type = p.as_str().to_string();
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
          aggregates: vec![],
          used_domain_objects: vec![],
        },
        BoundedContextDecl {
          name: "ShoppingCarContext".to_string(),
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
            field_type: "String".to_string(),
            initializer: None,
          },
          VariableDefinition {
            name: "price".to_string(),
            field_type: "Money".to_string(),
            initializer: None,
          }],
        value_objects: vec![],
      }],
      value_objects: vec![],
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
        BoundedContextDecl { name: "OrderContext".to_string(), aggregates: vec![], used_domain_objects: vec![] },
        BoundedContextDecl { name: "SalesContext".to_string(), aggregates: vec![], used_domain_objects: vec![] },
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
        field_type: "".to_string(),
        initializer: None,
      },
      inline_doc: "".to_string(),
      fields: vec![
        VariableDefinition { name: "id".to_string(), field_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "token".to_string(), field_type: "UUID".to_string(), initializer: None },
        VariableDefinition { name: "status".to_string(), field_type: "ReservationStatus".to_string(), initializer: Some("ReservationStatus.OPEN".to_string()) },
        VariableDefinition { name: "expiresAt".to_string(), field_type: "LocalDateTime".to_string(), initializer: None },
        VariableDefinition { name: "createdAt".to_string(), field_type: "LocalDateTime".to_string(), initializer: None },
        VariableDefinition { name: "screeningId".to_string(), field_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "screeningStartTime".to_string(), field_type: "LocalDateTime".to_string(), initializer: None },
        VariableDefinition { name: "name".to_string(), field_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "surname".to_string(), field_type: "String".to_string(), initializer: None },
        VariableDefinition { name: "tickets".to_string(), field_type: "Set<Ticket>".to_string(), initializer: None },
        VariableDefinition { name: "totalPrice".to_string(), field_type: "BigDecimal".to_string(), initializer: None }],
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
      })
    );
  }
}
