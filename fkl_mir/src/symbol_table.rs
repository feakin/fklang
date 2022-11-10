use std::collections::HashMap;

use crate::{Aggregate, BoundedContext, ContextMap, Entity, Implementation, ValueObject};
use crate::tactic::struct_::Struct;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolTable<'a> {
  /// symbol name start with DDD type name, for example:
  /// `Aggregate Ticket  {}` will have symbol name `aggregate_ticket`
  /// `Entity Ticket {}` will have symbol name `entity_ticket`
  /// `ValueObject Ticket {}` will have symbol name `value_object_ticket`
  pub symbols: HashMap<String, Symbol<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol<'a> {
  pub name: String,
  pub symbol_type: SymbolType<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType<'a> {
  ContextMap(Box<&'a ContextMap>),
  BoundedContext(Box<&'a BoundedContext>),
  Aggregate(Box<&'a Aggregate>),
  Entity(Box<&'a Entity>),
  ValueObject(Box<&'a ValueObject>),
  Struct(Box<&'a Struct>),
  Implementation(Box<&'a Implementation>),
  Environment,
}

impl<'a> Symbol<'a> {

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_symbol() {
     let mut symbol_table = SymbolTable::default();
      let context_map = ContextMap::default();
      let symbol = Symbol {
        name: "context_map".to_string(),
        symbol_type: SymbolType::ContextMap(Box::new(&context_map)),
      };
      symbol_table.symbols.insert(symbol.name.clone(), symbol);
      assert_eq!(symbol_table.symbols.len(), 1);
      assert_eq!(symbol_table.symbols.get("context_map").unwrap().name, "context_map");
  }
}
