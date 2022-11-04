use tree_sitter::{Node, QueryCapture};
use crate::code_meta::{CodeFile, CodeFunction, Location};

pub trait CodeConstruct {
  fn parse(code: &str) -> CodeFile;

  fn insert_location<T: Location>(model: &mut T, node: Node) {
    model.set_start(node.start_position().row, node.start_position().column);
    model.set_end(node.end_position().row, node.end_position().column);
  }

  fn insert_function(capture: QueryCapture, text: &str) -> CodeFunction {
    let mut function = CodeFunction::default();
    function.name = text.to_string();

    let node = capture.node.parent().unwrap();

    function.set_start(node.start_position().row, node.start_position().column);
    function.set_end(node.end_position().row, node.end_position().column);

    function
  }
}
