extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

pub use parser::parse as ast_parse;

use crate::mir::ContextMap;
use crate::parser::parse_result::ParseError;
use crate::transform::MirTransform;

pub mod parser;
pub mod mir;
pub mod resolve;

mod transform;
mod tests;
mod testing;

pub fn parse(rule_content: &str) -> Result<ContextMap, ParseError> {
  MirTransform::mir(rule_content)
}
