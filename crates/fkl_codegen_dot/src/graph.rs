use std::fmt;
use std::fmt::{Display, Formatter};

use crate::edge::Edge;
use crate::helper::config::indent;
use crate::node::Node;
use crate::subgraph::Subgraph;

// use domain_derive::Entity;

// #[derive(Entity)]
pub struct Graph {
  name: String,
  nodes: Vec<Node>,
  edges: Vec<Edge>,
  node_styles: Vec<String>,
  graph_style: Vec<String>,
  subgraph: Vec<Subgraph>,
}

impl Graph {
  pub fn new(name: &str) -> Self {
    Graph {
      name: name.to_string(),
      nodes: Vec::new(),
      edges: Vec::new(),
      node_styles: vec![],
      graph_style: vec![],
      subgraph: Vec::new(),
    }
  }

  pub fn add_node(&mut self, node: Node) {
    self.nodes.push(node);
  }

  pub fn add_edge(&mut self, source: &str, target: &str) {
    self.edges.push(Edge::new(source.to_string(), target.to_string()));
  }

  pub fn add_subgraph(&mut self, subgraph: Subgraph) {
    self.subgraph.push(subgraph);
  }

  pub fn set_name(&mut self, name: &str) {
    self.name = name.to_string();
  }

  pub fn add_node_style(&mut self, style: &str) {
    self.node_styles.push(style.to_string());
  }

  pub(crate) fn set_shape(&mut self, shape: &str) {
    self.add_node_style(&format!("shape={}", shape));
  }

  pub fn add_edge_with_style(&mut self, source: &str, target: &str, style: Vec<String>) {
    self.edges.push(Edge::styled(source.to_string(), target.to_string(), style));
  }

  pub(crate) fn set_style(&mut self, style: &str) {
    self.add_node_style(&format!("style={}", style));
  }

  pub fn use_default_style(&mut self) {
    self.set_shape("box");
    self.set_style("filled");

    self.graph_style.push("component=true".to_string());
  }
}

impl Display for Graph {
  fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
    let space = indent(1);
    out.write_str(&format!("digraph {} {{\n", self.name))?;

    if !self.graph_style.is_empty() {
      out.write_str(&format!("{}{};\n", space, self.graph_style.join("")))?;
    }

    if !self.node_styles.is_empty() {
      out.write_str(&format!("{}node [{}];\n", space, self.node_styles.join(" ")))?;
    }

    for node in &self.nodes {
      out.write_str(&format!("{}{}\n", space, node))?
    }

    for edge in &self.edges {
      out.write_str(&format!("{}{}\n", space, edge))?
    }

    for subgraph in &self.subgraph {
      out.write_str(&format!("\n{}\n", subgraph))?
    }

    out.write_str("}")
  }
}
