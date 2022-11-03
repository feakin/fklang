use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
  pub subdomain_type: SubDomainType
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubDomainType {
  Core,
  Generic,
  Supporting,
}

impl Default for SubDomainType {
  fn default() -> Self {
    SubDomainType::Supporting
  }
}
