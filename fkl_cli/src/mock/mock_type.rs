use indexmap::IndexMap;
use serde::{Deserialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::Serialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum MockType {
  Null,
  // Optional(Box<MockType>),
  /// basic type
  Unknown(String),
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
  /// structural type
  Array(Vec<MockType>),
  Map(IndexMap<String, MockType>),
  // additional type
  Date(String),
  DateTime(String),
  Timestamp(i64),
  Uuid(String),
}

#[allow(dead_code)]
impl MockType {
  pub fn integer(&self) -> i64 {
    match self {
      MockType::Integer(i) => *i,
      _ => panic!("cannot convert to integer"),
    }
  }

  pub fn float(&self) -> f64 {
    match self {
      MockType::Float(f) => *f,
      _ => panic!("cannot convert to float"),
    }
  }

  pub fn string(&self) -> String {
    match self {
      MockType::String(s) => s.clone(),
      _ => panic!("cannot convert to string"),
    }
  }

  pub(crate) fn boolean(&self) -> bool {
    match self {
      MockType::Boolean(b) => *b,
      _ => panic!("cannot convert to boolean"),
    }
  }

  pub(crate) fn datetime(&self) -> String {
    match self {
      MockType::DateTime(dt) => dt.to_string(),
      _ => panic!("cannot convert to datetime"),
    }
  }

  pub(crate) fn date(&self) -> String {
    match self {
      MockType::Date(d) => d.to_string(),
      _ => panic!("cannot convert to date"),
    }
  }

  pub(crate) fn timestamp(&self) -> i64 {
    match self {
      MockType::Timestamp(t) => *t,
      _ => panic!("cannot convert to timestamp"),
    }
  }

  pub(crate) fn uuid(&self) -> String {
    match self {
      MockType::Uuid(u) => u.clone(),
      _ => panic!("cannot convert to uuid"),
    }
  }
}

impl Serialize for MockType {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match self {
      MockType::Null => s.serialize_none(),
      MockType::Unknown(str) => s.serialize_str(str),
      MockType::String(str) => s.serialize_str(str),
      MockType::Integer(i) => s.serialize_i64(*i),
      MockType::Float(f) => s.serialize_f64(*f),
      MockType::Boolean(b) => s.serialize_bool(*b),
      MockType::Array(a) => {
        let mut seq = s.serialize_seq(Some(a.len()))?;
        for item in a {
          seq.serialize_element(item)?;
        }
        seq.end()
      }
      MockType::Map(m) => {
        let mut map = s.serialize_map(Some(m.len()))?;
        for (key, value) in m {
          map.serialize_entry(key, value)?;
        }
        map.end()
      }
      MockType::Date(d) => s.serialize_str(d),
      MockType::DateTime(dt) => s.serialize_str(dt),
      MockType::Timestamp(t) => s.serialize_i64(*t),
      MockType::Uuid(u) => s.serialize_str(u),
    }
  }
}

#[cfg(test)]
mod tests {
  use indexmap::IndexMap;
  use crate::mock::mock_type::MockType;

  #[test]
  fn test_serde_json() {
    let mut map: IndexMap<String, MockType> = IndexMap::new();
    map.insert("a".to_string(), MockType::String("b".to_string()));
    map.insert("c".to_string(), MockType::Integer(1));

    let s = serde_json::to_string(&map).unwrap();

    assert_eq!(s, r#"{"a":"b","c":1}"#);
  }

  #[test]
  fn nested_fake_value() {
    let mut map: IndexMap<String, MockType> = IndexMap::new();
    map.insert("a".to_string(), MockType::String("b".to_string()));
    map.insert("c".to_string(), MockType::Integer(1));

    let mut map2: IndexMap<String, MockType> = IndexMap::new();
    map2.insert("d".to_string(), MockType::String("e".to_string()));
    map2.insert("f".to_string(), MockType::Integer(2));
    map2.insert("g".to_string(), MockType::Map(map));

    let s = serde_json::to_string(&map2).unwrap();

    assert_eq!(s, r#"{"d":"e","f":2,"g":{"a":"b","c":1}}"#);
  }
}
