use serde::Deserialize;
use serde::Serialize;

// Basic, Digest, Bearer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationType {
  None,
  Basic(String, String),
  Digest(String, String),
  Bearer(String),
}

impl Default for AuthorizationType {
  fn default() -> Self {
    AuthorizationType::None
  }
}
