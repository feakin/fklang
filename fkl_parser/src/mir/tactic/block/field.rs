use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Field {
    pub name: String,
    pub initializer: Option<String>,
    pub type_type: String,
}
