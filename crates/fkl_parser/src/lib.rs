extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use parser::parse as ast_parse;

use crate::mir::ContextMap;
use crate::parser::parse_result::ParseError;
use crate::transform::Transform;

pub mod parser;
pub mod mir;

mod transform;
mod tests;

pub fn parse(rule_content: &str) -> Result<ContextMap, ParseError> {
  Ok(Transform::mir(rule_content)?)
}
