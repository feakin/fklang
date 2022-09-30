use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SourceSet {
  pub name: String,
  pub description: String,
  pub prefix: String,
  pub files: Vec<String>,
  pub source_set_type: SourceSetType,
}

impl SourceSet {
  pub fn new(name: &str) -> Self {
    SourceSet {
      name: name.to_string(),
      description: "".to_string(),
      prefix: "".to_string(),
      files: vec![],
      source_set_type: SourceSetType::default(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceSetType {
  None,
  StructUml,
  StructJsonSchema,
  StructProtobuf,
  StructAvro,
  OpenApi,
  UniqueLanguage,
}

impl Default for SourceSetType {
  fn default() -> Self {
    SourceSetType::None
  }
}
