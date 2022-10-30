use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum BuiltinType {
  Any,
  String,
  Integer,
  Float,
  Boolean,
  Date,
  DateTime,
  Timestamp,
  Array(Vec<BuiltinType>),
  Map(HashMap<String, BuiltinType>),
}


impl BuiltinType {
  pub fn from(s: &str) -> Self {
    let lower = s.to_lowercase();
    let s = lower.as_str();

    let is_array = s.starts_with("list<") && s.ends_with('>');
    if is_array {
      let inner = s[5..s.len() - 1].to_string();
      let inner_type = BuiltinType::from(&inner);
      return BuiltinType::Array(vec![inner_type]);
    }

    let is_map = s.starts_with("map<") && s.ends_with('>');
    if is_map {
      let inner = s[4..s.len() - 1].to_string();
      let inner_types: Vec<&str> = inner.split(',').collect();
      let key_type = BuiltinType::from(inner_types[0].trim());
      let value_type = BuiltinType::from(inner_types[1].trim());

      let mut map = HashMap::new();
      map.insert(key_type.to_string(), value_type);
      return BuiltinType::Map(map);
    }

    match s {
      "any" => BuiltinType::Any,
      "string" => BuiltinType::String,
      "int" => BuiltinType::Integer,
      "float" => BuiltinType::Float,
      "bool" => BuiltinType::Boolean,
      "date" => BuiltinType::Date,
      "datetime" => BuiltinType::DateTime,
      "timestamp" => BuiltinType::Timestamp,
      _ => BuiltinType::String,
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
      BuiltinType::Any => "any".to_owned(),
      BuiltinType::Date => "date".to_owned(),
      BuiltinType::DateTime => "datetime".to_owned(),
      BuiltinType::Timestamp => "timestamp".to_owned(),
      BuiltinType::Array(array) => {
        let mut s = "list<".to_owned();
        for item in array {
          s.push_str(&item.to_string());
          s.push_str(",");
        }
        s.pop();
        s.push('>');
        s
      }
      BuiltinType::Map(map) => {
        let mut s = "map<".to_owned();
        for (key, value) in map {
          s.push_str(&key);
          s.push_str(", ");
          s.push_str(&value.to_string());
          s.push_str(",");
        }
        s.pop();
        s.push('>');
        s
      }
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
    let t = BuiltinType::from(s);
    assert_eq!(t, BuiltinType::String);
    assert_eq!(t.to_string(), s);
  }

  #[test]
  fn test_builtin_type_array() {
    let s = "list<string>";
    let t = BuiltinType::from(s);
    assert_eq!(t, BuiltinType::Array(vec![BuiltinType::String]));
    assert_eq!(t.to_string(), s);
  }

  #[test]
  fn test_builtin_type_map() {
    let s = "map<string, string>";
    let t = BuiltinType::from(s);
    assert_eq!(t.to_string(), s);
  }
}
