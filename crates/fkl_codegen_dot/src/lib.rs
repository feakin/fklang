pub mod graph;
pub mod subgraph;
pub mod node;
pub mod edge;
pub mod helper;


#[cfg(test)]
mod tests {
  use crate::graph::Graph;
  use crate::node::Node;
  use crate::subgraph::Subgraph;

  #[test]
  fn basic_graph() {
    let mut graph = Graph::new("empty_graph");
    graph.add_node(Node::new("a"));

    let subgraph = Subgraph::new("empty_subgraph", "Empty Subgraph");
    graph.add_subgraph(subgraph);

    assert_eq!(format!("{}", graph), r#"digraph empty_graph {
  a [label="a"];

  subgraph cluster_empty_subgraph {
    label="Empty Subgraph";
  }
}"#);
  }

  #[test]
  fn nested_subgraph() {
    let mut graph = Graph::new("nested_subgraph");
    graph.add_node(Node::new("a"));

    let mut subgraph = Subgraph::new("empty_subgraph", "Empty Subgraph");
    subgraph.set_depth(1);
    subgraph.add_node(Node::new("b"));

    let mut nested_subgraph = Subgraph::new("nested_subgraph", "Nested Subgraph");
    nested_subgraph.add_node(Node::new("c"));
    nested_subgraph.set_depth(2);
    subgraph.add_subgraph(nested_subgraph);

    graph.add_subgraph(subgraph);


    assert_eq!(format!("{}", graph), r#"digraph nested_subgraph {
  a [label="a"];

  subgraph cluster_empty_subgraph {
    label="Empty Subgraph";
    b [label="b"];

    subgraph cluster_nested_subgraph {
      label="Nested Subgraph";
      c [label="c"];
    }
  }
}"#);
  }

  #[test]
  fn graph_with_edge() {
    let mut graph = Graph::new("graph_with_edge");
    graph.add_node(Node::new("a"));
    graph.add_node(Node::new("b"));
    graph.add_edge("a", "b");

    assert_eq!(format!("{}", graph), r#"digraph graph_with_edge {
  a [label="a"];
  b [label="b"];
  a -> b;
}"#);
  }

  #[test]
  fn graph_with_edge_and_subgraph() {
    let mut graph = Graph::new("graph_with_edge_and_subgraph");
    graph.add_node(Node::new("a"));
    graph.add_node(Node::new("b"));
    graph.add_edge("a", "b");

    let mut subgraph = Subgraph::new("empty_subgraph", "Empty Subgraph");
    subgraph.set_depth(1);
    subgraph.add_node(Node::new("c"));
    graph.add_subgraph(subgraph);

    assert_eq!(format!("{}", graph), r#"digraph graph_with_edge_and_subgraph {
  a [label="a"];
  b [label="b"];
  a -> b;

  subgraph cluster_empty_subgraph {
    label="Empty Subgraph";
    c [label="c"];
  }
}"#);
  }

  #[test]
  fn graph_width_rect_shape_style() {
    let mut graph = Graph::new("graph_width_rect_shape_style");
    graph.use_default_style();

    graph.add_node(Node::new("a"));

    assert_eq!(format!("{}", graph), r#"digraph graph_width_rect_shape_style {
  component=true;
  node [shape=box style=filled];
  a [label="a"];
}"#);
  }
}
