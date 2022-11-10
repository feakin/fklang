use serde::{Deserialize, Serialize};

use crate::Field;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}
