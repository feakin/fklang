use fkl_parser::mir::ContextRelationType;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct BcEdgeStyle {
  pub label: String,
  pub headlabel: String,
  pub taillabel: String,
}

const HEAD_LABEL: &'static str = "headlabel";

const TAIL_LABEL: &'static str = "taillabel";

pub(crate) fn generate_edge_style(source: &Vec<ContextRelationType>, target: &Vec<ContextRelationType>) -> BcEdgeStyle {
  let mut style = BcEdgeStyle {
    label: "".to_string(),
    headlabel: "".to_string(),
    taillabel: "".to_string(),
  };

  if source.len() > 0 {
    // pickup first ?
    style.label = source[0].to_string();
  }

  let source_symbols = collect_by_label(source, HEAD_LABEL);
  let target_symbols = collect_by_label(target, TAIL_LABEL);

  if source.len() > 0 {
    style.headlabel = source_symbols.join(", ");
  }

  if target.len() > 0 {
    style.taillabel = target_symbols.join(", ");
  }

  style
}

fn collect_by_label(target: &Vec<ContextRelationType>, label: &str) -> Vec<String> {
  target.iter().map(|t|
    map_from_type(t).get(label).unwrap_or(&"".to_string()).to_string()
  )
    .collect::<Vec<String>>()
}

fn map_from_type(context_type: &ContextRelationType) -> HashMap<String, String> {
  let mut style = HashMap::new();
  match context_type {
    ContextRelationType::None => {}
    ContextRelationType::SharedKernel => {}
    ContextRelationType::Partnership => {}
    ContextRelationType::SeparateWay => {}
    ContextRelationType::CustomerSupplier => {
      style.insert("headlabel".to_string(), "D".to_string());
      style.insert("taillabel".to_string(), "U".to_string());
    }
    ContextRelationType::Conformist => {
      style.insert("headlabel".to_string(), "D".to_string());
      style.insert("taillabel".to_string(), "U".to_string());
    }
    ContextRelationType::AntiCorruptionLayer => {
      style.insert("headlabel".to_string(), "D".to_string());
      style.insert("taillabel".to_string(), "U".to_string());
    }
    ContextRelationType::OpenHostService => {
      style.insert("headlabel".to_string(), "D".to_string());
      style.insert("taillabel".to_string(), "U".to_string());
    }
    ContextRelationType::PublishedLanguage => {
      style.insert("headlabel".to_string(), "D".to_string());
      style.insert("taillabel".to_string(), "U".to_string());
    }
    ContextRelationType::BigBallOfMud => {
      style.insert("headlabel".to_string(), "*".to_string());
      style.insert("taillabel".to_string(), "*".to_string());
    }
  }

  style
}

#[cfg(test)]
mod test {
  use fkl_parser::mir::ContextRelationType;
  use fkl_parser::parse;
    use crate::bc_edge_style::{BcEdgeStyle, generate_edge_style};
    use crate::dot_gen::to_dot;

  #[test]
  fn test_to_dot() {
    let context_map = parse(r#"
ContextMap {
  ShoppingCartContext -> MallContext;
  ShoppingCartContext <-> MallContext;
}
    "#).unwrap();
    let string = to_dot(&context_map);

    assert_eq!(string, r#"digraph  {
  node [shape=box style=filled];
  MallContext [label="MallContext"];
  ShoppingCartContext [label="ShoppingCartContext"];
  ShoppingCartContext -> MallContext;
  MallContext -> ShoppingCartContext;
  ShoppingCartContext -> MallContext;
}"#);
  }

  #[test]
  fn test_context_map_edge_style_shared_kernel() {
    let style = generate_edge_style(&vec![ContextRelationType::SharedKernel], &vec![ContextRelationType::SharedKernel]);
    assert_eq!(style, BcEdgeStyle {
      label: "SharedKernel".to_string(),
      headlabel: "".to_string(),
      taillabel: "".to_string(),
    });

    let style = generate_edge_style(&vec![ContextRelationType::AntiCorruptionLayer], &vec![]);
    assert_eq!(style, BcEdgeStyle {
      label: "AntiCorruptionLayer".to_string(),
      headlabel: "D".to_string(),
      taillabel: "".to_string(),
    });
  }

  #[test]
  fn with_relation() {
    let context_map = parse(r#"
ContextMap {
  ShoppingCartContext [acl] -> MallContext [oh] ;
  ShoppingCartContext <-> MallContext;
}
    "#).unwrap();

    let string = to_dot(&context_map);
    assert_eq!(string, r#"digraph  {
  node [shape=box style=filled];
  MallContext [label="MallContext"];
  ShoppingCartContext [label="ShoppingCartContext"];
  ShoppingCartContext -> MallContext [label="AntiCorruptionLayer",headlabel="D"];
  MallContext -> ShoppingCartContext;
  ShoppingCartContext -> MallContext;
}"#);
  }
}
