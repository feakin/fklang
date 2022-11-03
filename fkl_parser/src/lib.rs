extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use fkl_mir::ContextMap;
pub use parser::parse as ast_parse;

use crate::parser::parse_result::ParseError;
use crate::transform::MirTransform;

mod parser;
mod resolve;
mod transform;
mod tests;
mod testing;

/// compile the fkl source code into a ContextMap
/// ```rust
/// use fkl_parser::parse;
///
/// let source = r#"ContextMap Demo {
/// }
/// "#;
///
/// let context_map = parse(source).unwrap();
/// ```
pub fn parse(code: &str) -> Result<ContextMap, ParseError> {
  MirTransform::mir(code)
}
