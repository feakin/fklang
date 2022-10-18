use std::fs;
use std::path::PathBuf;

use log::info;

use fkl_codegen_java::gen_http_api;
use fkl_parser::mir::{ContextMap, Implementation};

use crate::construct::code_construct::CodeConstruct;
use crate::construct::java_construct::JavaConstruct;
use crate::exec;
use crate::exec::LayerMap;
use crate::exec::LayerPathBuilder;
use crate::inserter::inserter::JavaInserter;

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

pub fn code_gen_by_path(input_path: &PathBuf, filter_impl: Option<String>, base_path: &PathBuf) {
  let mir = exec::mir_from_file(input_path);
  code_gen_by_mir(&mir, filter_impl, base_path);
}

pub fn code_gen_by_mir(mir: &ContextMap, filter_impl: Option<String>, base_path: &PathBuf) {
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
