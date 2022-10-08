use serde::Deserialize;
use serde::Serialize;

use crate::mir::flow::step::Step;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Flow {
  pub inline_doc: String,
  pub steps: Vec<Step>,
}
