use std::fmt::{Display, Formatter};

use serde::Deserialize;
use serde::Serialize;

use crate::mir::tactic::aggregate::Aggregate;

// # Bounded Context
// A description of a boundary (typically a subsystem, or the work of a particular team) within
// which a particular model is defined and applicable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BoundedContext {
  pub name: String,
  pub aggregates: Vec<Aggregate>,
}

impl BoundedContext {
  pub fn new(name: &str) -> Self {
    BoundedContext { name: name.to_string(), aggregates: vec![] }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContextRelation {
  pub source: String,
  pub target: String,
  pub connection_type: ConnectionDirection,
  pub source_type: Vec<ContextRelationType>,
  pub target_type: Vec<ContextRelationType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionDirection {
  Undirected,
  // -->
  PositiveDirected,
  // <--
  NegativeDirected,
  // <->
  BiDirected,
}

impl Default for ConnectionDirection {
  fn default() -> Self {
    ConnectionDirection::Undirected
  }
}

impl ConnectionDirection {
  pub fn from_connection(str: &str) -> Self {
    match str {
      "Undirected" | "-" | "--" => ConnectionDirection::Undirected,
      "PositiveDirected" | "->" => ConnectionDirection::PositiveDirected,
      "NegativeDirected" | "<-" => ConnectionDirection::NegativeDirected,
      "BiDirected" | "<->" => ConnectionDirection::BiDirected,
      _ => ConnectionDirection::Undirected,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ContextRelationType {
  None,
  // Symmetric relation
  SharedKernel,
  Partnership,
  // Upstream Downstream
  CustomerSupplier,
  Conformist,
  AntiCorruptionLayer,
  OpenHostService,
  PublishedLanguage,
  SeparateWay,
  // added in book "DDD Reference"
  BigBallOfMud,
}

impl Default for ContextRelationType {
  fn default() -> Self {
    ContextRelationType::None
  }
}

impl Display for ContextRelationType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ContextRelationType::None => write!(f, "None"),
      ContextRelationType::SharedKernel => write!(f, "SharedKernel"),
      ContextRelationType::Partnership => write!(f, "Partnership"),
      ContextRelationType::CustomerSupplier => write!(f, "CustomerSupplier"),
      ContextRelationType::Conformist => write!(f, "Conformist"),
      ContextRelationType::AntiCorruptionLayer => write!(f, "AntiCorruptionLayer"),
      ContextRelationType::OpenHostService => write!(f, "OpenHostService"),
      ContextRelationType::PublishedLanguage => write!(f, "PublishedLanguage"),
      ContextRelationType::SeparateWay => write!(f, "SeparateWay"),
      ContextRelationType::BigBallOfMud => write!(f, "BigBallOfMud"),
    }
  }
}

impl ContextRelationType {
  pub fn list(types: &Vec<String>) -> Vec<ContextRelationType> {
    types
      .iter()
      .map(|t| match t.to_lowercase().as_str() {
        "sharedkernel" | "sk" => ContextRelationType::SharedKernel,
        "partnership" | "p" => ContextRelationType::Partnership,
        "customersupplier" | "cs" => ContextRelationType::CustomerSupplier,
        "conformist" | "c" => ContextRelationType::Conformist,
        "anticorruptionlayer" | "acl" => ContextRelationType::AntiCorruptionLayer,
        "openhostservice" | "ohs" => ContextRelationType::OpenHostService,
        "publishedlanguage" | "pl" => ContextRelationType::PublishedLanguage,
        "separateway" | "sw" => ContextRelationType::SeparateWay,
        "bigballofmud" | "bb" => ContextRelationType::BigBallOfMud,
        _ => {
          ContextRelationType::None
        }
      })
      .collect()
  }
}
