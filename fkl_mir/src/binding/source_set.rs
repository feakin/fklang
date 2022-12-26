use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SourceSets {
  pub name: String,
  pub source_sets: Vec<SourceSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SourceSet {
  pub name: String,
  pub description: String,
  pub parser: String,
  pub extension: String,
  pub src_dirs: Vec<String>,
  pub source_set_type: SourceSetType,
}

impl SourceSet {
  pub fn new(name: &str) -> Self {
    SourceSet {
      name: name.to_string(),
      parser: "".to_string(),
      description: "".to_string(),
      extension: "".to_string(),
      src_dirs: vec![],
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
  Csv,
}

impl Default for SourceSetType {
  fn default() -> Self {
    SourceSetType::None
  }
}

impl SourceSetType {
  pub fn from_str(s: &str) -> Self {
    match s.to_lowercase().as_str() {
      "uml" => SourceSetType::StructUml,
      "puml" => SourceSetType::StructUml,
      "json_schema" => SourceSetType::StructJsonSchema,
      "protobuf" => SourceSetType::StructProtobuf,
      "avro" => SourceSetType::StructAvro,
      "open_api" => SourceSetType::OpenApi,
      "csv" => SourceSetType::Csv,
      _ => SourceSetType::None,
    }
  }
}
