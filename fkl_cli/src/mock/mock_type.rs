use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FakeValue {
  Null,
  // Optional(Box<MockType>),
  /// basic type
  Unknown(String),
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
  /// structural type
  Array(Vec<FakeValue>),
  Map(IndexMap<String, FakeValue>),
  // additional type
  Date(String),
  DateTime(String),
  Timestamp(i64),
  Uuid(String),
}

#[allow(dead_code)]
impl FakeValue {
  pub fn integer(&self) -> i64 {
    match self {
      FakeValue::Integer(i) => *i,
      _ => panic!("cannot convert to integer"),
    }
  }

  pub fn float(&self) -> f64 {
    match self {
      FakeValue::Float(f) => *f,
      _ => panic!("cannot convert to float"),
    }
  }

  pub fn string(&self) -> String {
    match self {
      FakeValue::String(s) => s.clone(),
      _ => panic!("cannot convert to string"),
    }
  }

  pub(crate) fn boolean(&self) -> bool {
    match self {
      FakeValue::Boolean(b) => *b,
      _ => panic!("cannot convert to boolean"),
    }
  }

  pub(crate) fn datetime(&self) -> String {
    match self {
      FakeValue::DateTime(dt) => dt.to_string(),
      _ => panic!("cannot convert to datetime"),
    }
  }

  pub(crate) fn date(&self) -> String {
    match self {
      FakeValue::Date(d) => d.to_string(),
      _ => panic!("cannot convert to date"),
    }
  }

  pub(crate) fn timestamp(&self) -> i64 {
    match self {
      FakeValue::Timestamp(t) => *t,
      _ => panic!("cannot convert to timestamp"),
    }
  }

  pub(crate) fn uuid(&self) -> String {
    match self {
      FakeValue::Uuid(u) => u.clone(),
      _ => panic!("cannot convert to uuid"),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::mock::mock_type::FakeValue;

  #[test]
  fn test_serde_json() {
    let mut map: HashMap<String, FakeValue> = HashMap::new();
    map.insert("a".to_string(), FakeValue::String("b".to_string()));
    map.insert("c".to_string(), FakeValue::Integer(1));

    let s = serde_json::to_string(&map).unwrap();

    assert_eq!(s, r#"{"a":{"String":"b"},"c":{"Integer":1}}"#);
  }
}
