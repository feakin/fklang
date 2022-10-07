use serde::Deserialize;
use serde::Serialize;

use crate::mir::tactic::block::Field;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ValueObject {
    pub name: String,
    pub fields: Vec<Field>,
}
