use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum BuiltinType {
  Any,
  String,
  Integer,
  Float,
  Boolean,
  Array,
  Map,
  Object,
  Date,
  DateTime,
  Timestamp,
}


impl BuiltinType {
  pub fn from_string(s: &str) -> Self {
    match s {
      "any" => BuiltinType::Any,
      "string" => BuiltinType::String,
      "int" => BuiltinType::Integer,
      "float" => BuiltinType::Float,
      "bool" => BuiltinType::Boolean,
      "list" => BuiltinType::Array,
      "map" => BuiltinType::Map,
      "object" => BuiltinType::Object,
      "date" => BuiltinType::Date,
      "datetime" => BuiltinType::DateTime,
      "timestamp" => BuiltinType::Timestamp,
      _ => panic!("unknown builtin type: {}", s),
    }
  }
}

impl ToString for BuiltinType {
  fn to_string(&self) -> String {
    match self {
      BuiltinType::String => "string".to_owned(),
      BuiltinType::Integer => "int".to_owned(),
      BuiltinType::Float => "float".to_owned(),
      BuiltinType::Boolean => "bool".to_owned(),
      BuiltinType::Array => "list".to_owned(),
      BuiltinType::Map => "map".to_owned(),
      BuiltinType::Object => "object".to_owned(),
      BuiltinType::Any => "any".to_owned(),
      BuiltinType::Date => "date".to_owned(),
      BuiltinType::DateTime => "datetime".to_owned(),
      BuiltinType::Timestamp => "timestamp".to_owned(),
    }
  }
}

impl Default for BuiltinType {
  fn default() -> Self {
    BuiltinType::Any
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_builtin_type() {
    let s = "string";
    let t = BuiltinType::from_string(s);
    assert_eq!(t, BuiltinType::String);
    assert_eq!(t.to_string(), s);
  }
}
