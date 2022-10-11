use std::fs;
use std::path::PathBuf;

use log::{error, info};

use fkl_codegen_java::gen_http_api;
use fkl_parser::mir::{ContextMap, Implementation};
use fkl_parser::parse;

use crate::exec::layer_map::LayerMap;
use crate::ident::base_ident::CodeIdent;
use crate::ident::java_ident::JavaIdent;
use crate::inserter::inserter::JavaInserter;

pub struct CodeBlock {
  pub target_layer: DddLayer,
  pub class_name: String,
  pub code: String,
}

pub enum DddLayer {
  Interface,
  Application,
  Domain,
  Infrastructure,
}

pub fn code_gen_exec(input_path: &PathBuf, filter_impl: Option<&String>, base_path: &PathBuf) {
  let feakin = fs::read_to_string(input_path).unwrap();
  let mir: ContextMap = parse(&feakin).or_else(|e| {
    error!("{}", e);
    Err(e)
  }).unwrap();

  let code_blocks = collect_codes(filter_impl, &mir);
  let has_layered_define = mir.layered.is_some();
  if !code_blocks.is_empty() {
    if has_layered_define {
      let layer_map = LayerMap::from(mir.layered.clone().unwrap());
      code_blocks.iter().for_each(|code_block| {
        let path = build_path(base_path, &layer_map, code_block);

        let code = fs::read_to_string(&path).unwrap();
        let code_file = JavaIdent::parse(&code);
        let first_class = &code_file.classes[0];

        let lines: Vec<String> = code_block.code.split("\n").map(|s| s.to_string()).collect();
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

fn build_path(base_path: &PathBuf, layer_map: &LayerMap, code_block: &CodeBlock) -> String {
  let file_name = code_block.class_name.clone() + "Controller.java";
  let mut target_path = base_path.clone();
  target_path.push(layer_map.interface_path().clone());
  target_path.push(file_name);

  if !target_path.exists() {
    panic!("target file not found: {}", target_path.to_str().unwrap());
  }

  format!("{}", target_path.display())
}

/// collect codes for generate.
fn collect_codes(filter_impl: Option<&String>, mir: &ContextMap) -> Vec<CodeBlock> {
  let mut codes: Vec<CodeBlock> = vec![];
  mir.implementations.iter()
    .for_each(|implementation| {
      match implementation {
        Implementation::PublishHttpApi(http) => {
          if let Some(filter_impl) = filter_impl {
            if &http.name == filter_impl {
              let output = gen_http_api(http, "java");
              codes.push(CodeBlock {
                target_layer: DddLayer::Interface,
                class_name: http.target(),
                code: output,
              });
            }
          } else {
            let output = gen_http_api(http, "java");
            codes.push(CodeBlock {
              target_layer: DddLayer::Interface,
              class_name: http.target(),
              code: output,
            });
          }
        }
        Implementation::PublishEvent => {}
        Implementation::PublishMessage => {}
      }
    });

  codes
}
