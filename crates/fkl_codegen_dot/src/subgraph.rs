use std::fmt;

use crate::edge::Edge;
use crate::helper::config::indent;
use crate::helper::naming::naming;
use crate::node::Node;

pub struct Subgraph {
  name: String,
  label: String,
  // for indent
  depth: usize,
  nodes: Vec<Node>,
  edges: Vec<Edge>,
  subgraph: Vec<Subgraph>,
}

impl Subgraph {
  pub fn new(name: &str, label: &str) -> Self {
    Subgraph {
      name: naming(name),
      label: label.to_string(),
      depth: 1,
      nodes: Vec::new(),
      edges: Vec::new(),
      subgraph: Vec::new(),
    }
  }

  pub fn add_subgraph(&mut self, subgraph: Subgraph) {
    self.subgraph.push(subgraph);
  }

  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }

  pub fn set_depth(&mut self, depth: usize) {
    self.depth = depth;
  }
}

impl fmt::Display for Subgraph {
  fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
    out.write_str(&format!("{}subgraph {} {{\n", indent(self.depth), self.name))?;

    let space = indent(self.depth + 1);

    out.write_str(&format!("{}label=\"{}\";\n", space, self.label))?;

    for node in &self.nodes {
      out.write_str(&format!("{}{}\n", space, node))?
    }

    for edge in &self.edges {
      out.write_str(&format!("{}{}\n", space, edge))?
    }

    for subgraph in &self.subgraph {
      out.write_str(&format!("\n{}\n", subgraph))?
    }

    out.write_str(&format!("{}}}", indent(self.depth)))
  }
}
