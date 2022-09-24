use std::fmt;
use std::fmt::{Formatter};

pub struct Edge {
  from: String,
  to: String,
  style: Vec<String>,
}

impl Edge {
  pub fn new(from: String, to: String) -> Self {
    Edge { from, to, style: vec![] }
  }
  pub fn styled(from: String, to: String, style: Vec<String>) -> Self {
    Edge { from, to, style }
  }
}

impl fmt::Display for Edge {
  fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
    if self.style.len() > 0 {
      out.write_str(&format!("{} -> {} [{}];", self.from, self.to, self.style.join(",")))
    } else {
      out.write_str(&format!("{} -> {};", self.from, self.to))
    }
  }
}
