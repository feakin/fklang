use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct Node {
  name: String,
  label: String,
}

impl Node {
  pub fn new(name: &str) -> Self {
    Node { name: name.to_string(), label: name.to_string() }
  }
}

impl fmt::Display for Node {
  fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
    out.write_str(&format!("{} [label=\"{}\"];", self.name, self.label))
  }
}
