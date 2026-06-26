use std::fs;
use std::path::PathBuf;

use log::info;

use fkl_codegen_java::gen_http_api;
use fkl_codegen_sql::gen_schema;
use fkl_mir::{ContextMap, Implementation};

use crate::builtin::funcs;
use crate::builtin::funcs::LayerMap;
use crate::builtin::funcs::LayerPathBuilder;
use crate::deconstruct::code_construct::CodeConstruct;
use crate::deconstruct::java_construct::JavaConstruct;
use crate::inserter::inserter::Inserter;
use crate::inserter::java_inserter::JavaInserter;
use crate::SupportedFramework;

pub mod layer_map;
pub mod layer_path_builder;

pub struct CodeBlock {
  pub target_layer: DddLayer,
  pub class_name: String,
  pub method_name: String,
  pub code: String,
}

pub enum DddLayer {
  Interface,
  Application,
  Domain,
  Infrastructure,
}

pub fn code_gen_by_path(
  input_path: &PathBuf,
  filter_impl: Option<String>,
  base_path: &PathBuf,
  framework: &SupportedFramework,
) {
  let mir = funcs::mir_from_file(input_path);
  code_gen_by_mir(&mir, filter_impl, base_path, framework);
}

// todo: extract to a separate module
pub fn code_gen_by_mir(
  mir: &ContextMap,
  filter_impl: Option<String>,
  base_path: &PathBuf,
  framework: &SupportedFramework,
) {
  if framework == &SupportedFramework::Sql {
    let output_path = base_path.join("schema.sql");
    fs::write(&output_path, gen_schema(mir)).expect("failed to write schema.sql");
    info!("generated sql schema to {}", output_path.display());
    return;
  }

  let code_blocks = collect_codes(filter_impl, &mir);
  let has_layered_define = mir.layered.is_some();
  if !code_blocks.is_empty() {
    if has_layered_define {
      let layer_map = LayerMap::from(mir.layered.clone().unwrap());
      code_blocks.iter().for_each(|block| {
        let path = LayerPathBuilder::controller(base_path, &layer_map, block.class_name.clone());

        let code = fs::read_to_string(&path).unwrap();
        let code_file = JavaConstruct::parse(&code);
        let first_class = &code_file.classes[0];

        if first_class.is_contain_method(&block.method_name) {
          panic!("{} already has method {}", block.class_name, block.method_name);
        }

        let lines: Vec<String> = block.code.split("\n").map(|s| s.to_string()).collect();
        JavaInserter::insert(&path, first_class, &lines)
          .expect("TODO: panic message");

        info!("inserted to {}, code: {}", path, &lines.join("\n"));
      });
    } else {
      code_blocks.iter().for_each(|block| {
        info!("no layered define found, generate code {}", block.code);
      });
    }
  }
}

/// collect codes for generate.
fn collect_codes(filter_impl: Option<String>, mir: &ContextMap) -> Vec<CodeBlock> {
  let mut codes: Vec<CodeBlock> = vec![];
  mir.implementations.iter()
    .for_each(|implementation| {
      match implementation {
        Implementation::PublishHttpApi(http) => {
          if let Some(filter_impl) = &filter_impl {
            if &http.name == filter_impl {
              let output = gen_http_api(http, "java");
              codes.push(CodeBlock {
                target_layer: DddLayer::Interface,
                class_name: http.target(),
                method_name: output.method_name.clone(),
                code: output.code,
              });
            }
          } else {
            let output = gen_http_api(http, "java");
            codes.push(CodeBlock {
              target_layer: DddLayer::Interface,
              class_name: http.target(),
              method_name: output.method_name.clone(),
              code: output.code,
            });
          }
        }
        Implementation::PublishEvent => {}
        Implementation::PublishMessage => {}
      }
    });

  codes
}

#[cfg(test)]
mod tests {
  use std::time::{SystemTime, UNIX_EPOCH};

  use fkl_mir::{Aggregate, BoundedContext, Entity, Field};

  use super::*;

  #[test]
  fn sql_framework_writes_schema_file() {
    let mut ticket = Entity::new("Ticket");
    ticket.fields = vec![
      Field { name: "id".to_string(), type_type: "UUID".to_string(), initializer: None },
      Field { name: "price".to_string(), type_type: "Int".to_string(), initializer: None },
    ];
    let mir = ContextMap {
      name: "Ticketing".to_string(),
      contexts: vec![BoundedContext {
        name: "Sales".to_string(),
        aggregates: vec![Aggregate {
          name: "TicketSale".to_string(),
          entities: vec![ticket],
          ..Default::default()
        }],
      }],
      ..Default::default()
    };
    let output_dir = std::env::temp_dir().join(format!(
      "fkl-codegen-sql-{}-{}",
      std::process::id(),
      SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&output_dir).unwrap();

    code_gen_by_mir(&mir, None, &output_dir, &SupportedFramework::Sql);

    let schema = fs::read_to_string(output_dir.join("schema.sql")).unwrap();
    fs::remove_dir_all(&output_dir).unwrap();
    assert_eq!(schema, "CREATE TABLE ticket (\n  id UUID,\n  price INTEGER\n);\n");
  }
}
