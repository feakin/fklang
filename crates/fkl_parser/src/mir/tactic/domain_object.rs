use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DomainObjectType {
  ApplicationService,
  AggregateRoot,
  DomainEvent,
  Entity,
  ValueObject,
}

impl Default for DomainObjectType {
  fn default() -> Self {
    DomainObjectType::ValueObject
  }
}

pub trait DomainObject {
  fn name(&self) -> &str;
  fn inline_doc(&self) -> &str;
  fn object_type(&self) -> DomainObjectType;

  fn is_aggregate_root(&self) -> bool;
  fn has_unique_id(&self) -> bool;
}
