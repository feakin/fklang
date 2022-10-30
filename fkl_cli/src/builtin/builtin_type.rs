use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum BuiltinType {
  String,
  Int,
  Float,
  Bool,
  List,
  Map,
  Object,
  Any,
}


impl BuiltinType {
  pub fn from_string(s: &str) -> Self {
    match s {
      "string" => BuiltinType::String,
      "int" => BuiltinType::Int,
      "float" => BuiltinType::Float,
      "bool" => BuiltinType::Bool,
      "list" => BuiltinType::List,
      "map" => BuiltinType::Map,
      "object" => BuiltinType::Object,
      "any" => BuiltinType::Any,
      _ => panic!("unknown builtin type {}", s),
    }
  }
}

impl ToString for BuiltinType {
  fn to_string(&self) -> String {
    match self {
      BuiltinType::String => "string".to_owned(),
      BuiltinType::Int => "int".to_owned(),
      BuiltinType::Float => "float".to_owned(),
      BuiltinType::Bool => "bool".to_owned(),
      BuiltinType::List => "list".to_owned(),
      BuiltinType::Map => "map".to_owned(),
      BuiltinType::Object => "object".to_owned(),
      BuiltinType::Any => "any".to_owned(),
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
