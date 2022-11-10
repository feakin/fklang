use std::collections::HashMap;

use crate::{Aggregate, BoundedContext, ContextMap, Datasource, Entity, Environment, Implementation, SourceSet, ValueObject};
use crate::tactic::struct_::Struct;

/// SymbolType combines all DDD types and some other top level types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolType {
  ContextMap(ContextMap),
  BoundedContext(BoundedContext),
  Aggregate(Aggregate),
  Entity(Entity),
  ValueObject(ValueObject),
  Struct(Struct),
  Implementation(Implementation),
  Environment(Environment),
  SourceSet(SourceSet),
  DataSource(Datasource),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SymbolTable {
  pub symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
  pub fn new() -> Self {
    SymbolTable { symbols: HashMap::new() }
  }

  pub fn add(&mut self, symbol_type: SymbolType) {
    let name = Symbol::name(&symbol_type);
    self.symbols.insert(name.clone(), Symbol { name, symbol_type });
  }

  pub fn get(&self, name: &str) -> Option<&Symbol> {
    self.symbols.get(name)
  }
}

/// Symbol is a DDD type or a top level type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
  pub name: String,
  pub symbol_type: SymbolType,
}

impl Symbol {
  /// symbol name start with DDD type name, for example:
  /// `Aggregate Ticket  {}` will have symbol name `aggregate_ticket`
  /// `Entity Ticket {}` will have symbol name `entity_ticket`
  /// `ValueObject Ticket {}` will have symbol name `value_object_ticket`
  pub fn new(symbol_type: SymbolType) -> Self {
    let name = Self::name(&symbol_type);
    Symbol { name, symbol_type }
  }

  fn name(symbol_type: &SymbolType) -> String {
    let name = match &symbol_type {
      SymbolType::ContextMap(map) => format!("context_map_{}", map.name),
      SymbolType::BoundedContext(bc) => format!("bounded_context_{}", bc.name),
      SymbolType::Aggregate(aggregate) => format!("aggregate_{}", aggregate.name),
      SymbolType::Entity(entity) => format!("entity_{}", entity.name),
      SymbolType::ValueObject(vo) => format!("value_object_{}", vo.name),
      SymbolType::Struct(struct_) => format!("struct_{}", struct_.name),
      SymbolType::Implementation(implementation) => format!("implementation_{}", implementation.name()),
      SymbolType::Environment(environment) => format!("environment_{}", environment.name),
      SymbolType::SourceSet(source_set) => format!("source_set_{}", source_set.name),
      SymbolType::DataSource(data_source) => format!("data_source_{}", data_source.name()),
    };
    name
  }
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
      symbol_type: SymbolType::ContextMap(context_map),
    };
    symbol_table.symbols.insert(symbol.name.clone(), symbol);
    assert_eq!(symbol_table.symbols.len(), 1);
    assert_eq!(symbol_table.symbols.get("context_map").unwrap().name, "context_map");
  }

  #[test]
  fn test_symbol_table() {
    let mut symbol_table = SymbolTable::default();
    let context_map = ContextMap::default();

    symbol_table.add(SymbolType::ContextMap(context_map));
    assert_eq!(symbol_table.symbols.len(), 1);
    assert_eq!(symbol_table.symbols.get("context_map_").unwrap().name, "context_map_");
  }

  #[test]
  fn add_symbol_by_new() {
    let mut symbol_table = SymbolTable::default();
    let mut context_map = ContextMap::default();
    context_map.name = "demo".to_string();

    symbol_table.add(SymbolType::ContextMap(context_map));
    assert_eq!(symbol_table.symbols.len(), 1);

    assert_eq!(symbol_table.symbols.get("context_map_demo").unwrap().name, "context_map_demo");
  }
}
