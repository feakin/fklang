use std::collections::HashMap;

use crate::builtin::builtin_type::BuiltinType;

#[derive(Debug, Clone, PartialEq)]
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
  Array(Vec<BuiltinType>),
  Map(Vec<(BuiltinType, BuiltinType)>),
  Object(HashMap<String, BuiltinType>),
  // additional type
  Date(chrono::Date<chrono::Utc>),
  DateTime(chrono::DateTime<chrono::Utc>),
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

  pub(crate) fn datetime(&self) -> chrono::DateTime<chrono::Utc> {
    match self {
      FakeValue::DateTime(dt) => dt.clone(),
      _ => panic!("cannot convert to datetime"),
    }
  }

  pub(crate) fn date(&self) -> chrono::Date<chrono::Utc> {
    match self {
      FakeValue::Date(d) => d.clone(),
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
