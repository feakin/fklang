use std::collections::HashMap;

use crate::builtin::builtin_type::BuiltinType;

#[derive(Debug, Clone, PartialEq)]
pub enum MockType {
  Null,
  Optional(Box<MockType>),
  /// basic type
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

  pub(crate) fn datetime(&self) -> chrono::DateTime<chrono::Utc> {
    match self {
      MockType::DateTime(dt) => dt.clone(),
      _ => panic!("cannot convert to datetime"),
    }
  }

  pub(crate) fn date(&self) -> chrono::Date<chrono::Utc> {
    match self {
      MockType::Date(d) => d.clone(),
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
