use serde::Deserialize;
use serde::Serialize;

use crate::mir::implementation::api_contract::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Implementation {
  pub name: String,
  pub description: String,
  pub domain_bindings: Vec<DomainEventBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DomainEventBinding {
  pub name: String,
  pub description: String,
  pub domain_event: String,
  // todo: thinking in a better way to do this
  pub api_contract: Endpoint,
}
