use fkl_dot::graph::Graph;
use fkl_dot::helper::naming::cluster_name;
use fkl_dot::node::Node;
use fkl_dot::subgraph::Subgraph;
use fkl_parser::mir::{ConnectionDirection, ContextMap, ContextRelation};

use crate::bc_edge_style;
use crate::bc_edge_style::BcEdgeStyle;

pub(crate) fn to_dot(context_map: &ContextMap) -> String {
  let mut graph = Graph::new(&context_map.name);
  graph.use_default_style();

  for bc in &context_map.contexts {
    let name = &bc.name;
    let mut subgraph = Subgraph::new(&bc.name, &format!("{}(Context)", name));
    for aggregate in &bc.aggregates {
      let mut aggregate_graph = Subgraph::new(&format!("aggregate_{}", aggregate.name), &format!("{}(Aggregate)", aggregate.name));

      aggregate.entities.iter().for_each(|entity| {
        aggregate_graph.add_node(Node::label(&format!("entity_{}", entity.name), &entity.name));
      });

      subgraph.add_subgraph(aggregate_graph);
    }

    graph.add_subgraph(subgraph);
  }

  for rel in &context_map.relations {
    process_context_edge(&mut graph, rel);
  }

  format!("{}", graph)
}

fn process_context_edge(graph: &mut Graph, relation: &ContextRelation) {
  let bc_edge_style = bc_edge_style::generate_edge_style(&relation.source_type, &relation.target_type);
  let style = create_graph_edge_style(bc_edge_style);

  let source = &cluster_name(&relation.source);
  let target = &cluster_name(&relation.target);

  match &relation.connection_type {
    ConnectionDirection::Undirected => {}
    ConnectionDirection::PositiveDirected => {
      graph.add_edge_with_style(source, target, style);
    }
    ConnectionDirection::NegativeDirected => {
      graph.add_edge_with_style(target, source, style);
    }
    ConnectionDirection::BiDirected => {
      graph.add_edge_with_style(target, source, style);
      graph.add_edge(source, target);
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

#[cfg(test)]
mod test {
  use fkl_parser::parse;

  use crate::dot_gen::to_dot;

  #[test]
  fn nested_entity() {
    let input = r#"
ContextMap TicketBooking {
  Reservation -> Cinema;
  Reservation -> Movie;
  Reservation -> User;
}

Context Reservation {
  Aggregate Reservation;
}

Aggregate Reservation {
  Entity Ticket, Reservation;
}
"#;

    let context_map = parse(input).unwrap();
    let dot = to_dot(&context_map);
    assert_eq!(dot, r#"digraph TicketBooking {
  component=true;layout=fdp;
  node [shape=box style=filled];
  cluster_reservation -> cluster_cinema;
  cluster_reservation -> cluster_movie;
  cluster_reservation -> cluster_user;

  subgraph cluster_cinema {
    label="Cinema(Context)";
  }

  subgraph cluster_movie {
    label="Movie(Context)";
  }

  subgraph cluster_reservation {
    label="Reservation(Context)";

  subgraph cluster_aggregate_reservation {
    label="Reservation(Aggregate)";
    entity_Ticket [label="Ticket"];
    entity_Reservation [label="Reservation"];
  }
  }

  subgraph cluster_user {
    label="User(Context)";
  }
}"#);
  }
}
