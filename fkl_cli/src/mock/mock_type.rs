use std::collections::HashMap;

use rocket::serde::{Deserialize, Serialize};

use crate::builtin::builtin_type::BuiltinType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MockType {
  Null,
  Optional(Box<MockType>),
  /// basic type
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
  Char(char),
  /// structural type
  Array(Vec<BuiltinType>),
  Map(Vec<(BuiltinType, BuiltinType)>),
  Object(HashMap<String, BuiltinType>),
  // additional type
  Date(String),
  DateTime(String),
  Timestamp(String),
  Uuid(String),
}

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
}
