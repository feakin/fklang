use indexmap::IndexMap;
use serde::{Deserialize, Serializer};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::Serialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
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

impl Serialize for FakeValue {
  fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match self {
      FakeValue::Null => s.serialize_none(),
      FakeValue::Unknown(str) => s.serialize_str(str),
      FakeValue::String(str) => s.serialize_str(str),
      FakeValue::Integer(i) => s.serialize_i64(*i),
      FakeValue::Float(f) => s.serialize_f64(*f),
      FakeValue::Boolean(b) => s.serialize_bool(*b),
      FakeValue::Array(a) => {
        let mut seq = s.serialize_seq(Some(a.len()))?;
        for item in a {
          seq.serialize_element(item)?;
        }
        seq.end()
      },
      FakeValue::Map(m) => {
        let mut map = s.serialize_map(Some(m.len()))?;
        for (key, value) in m {
          map.serialize_entry(key, value)?;
        }
        map.end()
      },
      FakeValue::Date(d) => s.serialize_str(d),
      FakeValue::DateTime(dt) => s.serialize_str(dt),
      FakeValue::Timestamp(t) => s.serialize_i64(*t),
      FakeValue::Uuid(u) => s.serialize_str(u),
    }
  }
}

#[cfg(test)]
mod tests {
  use indexmap::IndexMap;
  use crate::mock::mock_type::FakeValue;

  #[test]
  fn test_serde_json() {
    let mut map: IndexMap<String, FakeValue> = IndexMap::new();
    map.insert("a".to_string(), FakeValue::String("b".to_string()));
    map.insert("c".to_string(), FakeValue::Integer(1));

    let s = serde_json::to_string(&map).unwrap();

    assert_eq!(s, r#"{"a":"b","c":1}"#);
  }

  #[test]
  fn nested_fake_value() {
    let mut map: IndexMap<String, FakeValue> = IndexMap::new();
    map.insert("a".to_string(), FakeValue::String("b".to_string()));
    map.insert("c".to_string(), FakeValue::Integer(1));

    let mut map2: IndexMap<String, FakeValue> = IndexMap::new();
    map2.insert("d".to_string(), FakeValue::String("e".to_string()));
    map2.insert("f".to_string(), FakeValue::Integer(2));
    map2.insert("g".to_string(), FakeValue::Map(map));

    let s = serde_json::to_string(&map2).unwrap();

    assert_eq!(s, r#"{"d":"e","f":2,"g":{"a":"b","c":1}}"#);
  }
}
