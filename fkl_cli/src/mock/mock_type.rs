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
