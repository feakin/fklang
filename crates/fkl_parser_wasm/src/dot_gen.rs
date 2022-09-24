use fkl_dot::graph::Graph;
use fkl_dot::node::Node;
use fkl_parser::mir::{ConnectionDirection, ContextMap, ContextRelation};
use crate::bc_edge_style;
use crate::bc_edge_style::BcEdgeStyle;

pub(crate) fn to_dot(context_map: &ContextMap) -> String {
  let mut graph = Graph::new(&context_map.name);
  graph.use_default_style();

  for bc in &context_map.contexts {
    graph.add_node(Node::new(&bc.name))
  }

  for rel in &context_map.relations {
    process_context_edge(&mut graph, rel);
  }

  format!("{}", graph)
}

fn process_context_edge(graph: &mut Graph, relation: &ContextRelation) {
  let bc_edge_style = bc_edge_style::generate_edge_style(&relation.source_type, &relation.target_type);
  let style = create_graph_edge_style(bc_edge_style);

  match &relation.connection_type {
    ConnectionDirection::Undirected => {}
    ConnectionDirection::PositiveDirected => {
      // add_edge_labels(graph, &relation.source, &relation.target);
      graph.add_edge_with_style(&relation.source, &relation.target, style);
    }
    ConnectionDirection::NegativeDirected => {
      graph.add_edge_with_style(&relation.target, &relation.source, style);
    }
    ConnectionDirection::BiDirected => {
      graph.add_edge_with_style(&relation.target, &relation.source, style);
      graph.add_edge(&relation.source, &relation.target);
    }
  }
}

fn create_graph_edge_style(bc_style: BcEdgeStyle) -> Vec<String> {
  let mut style = vec![];
  if bc_style.label.len() > 0 {
    style.push(format!("label=\"{}\"", bc_style.label));
  }

  if bc_style.headlabel.len() > 0 {
    style.push(format!("headlabel=\"{}\"", bc_style.headlabel));
  }

  if bc_style.taillabel.len() > 0 {
    style.push(format!("taillabel=\"{}\"", bc_style.taillabel));
  }
  style
}
