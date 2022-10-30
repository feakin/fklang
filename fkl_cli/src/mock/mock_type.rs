use std::collections::HashMap;

use rocket::serde::{Deserialize, Serialize};

use crate::builtin::builtin_type::BuiltinType;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MockType {
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
  Char(char),
  // also custom
  Array(Vec<BuiltinType>),
  Map(Vec<(BuiltinType, BuiltinType)>),
  Object(HashMap<String, BuiltinType>),
  Null,
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
}
