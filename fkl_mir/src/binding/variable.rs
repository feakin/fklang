use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct VariableDefinition {
  pub name: String,
  pub type_type: String,
  pub initializer: Option<String>,
}
