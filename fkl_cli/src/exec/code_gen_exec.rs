use std::fs;
use std::path::PathBuf;

use log::error;

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

pub fn code_gen_exec(feakin_path: Option<&PathBuf>, filter_impl: Option<&String>, base_path: &PathBuf) {
  if feakin_path.is_none() {
    error!("Please provide a path to a manifest file");
    return;
  }

  let path = feakin_path.unwrap();
  let feakin = fs::read_to_string(path).unwrap();
  let mir: ContextMap = parse(&feakin).or_else(|e| {
    error!("{}", e);
    Err(e)
  }).unwrap();

  let code_blocks = collect_codes(filter_impl, &mir);
  let has_layered_define = mir.layered.is_some();
  if !code_blocks.is_empty() && has_layered_define {
    let layer_map = LayerMap::from(mir.layered.clone().unwrap());

    code_blocks.iter().for_each(|code_block| {
      let file_name = code_block.class_name.clone() + "Controller.java";
      let mut target_path = base_path.clone();
      target_path.push(layer_map.interface_path().clone());
      target_path.push(file_name);

      if !target_path.exists() {
        panic!("target file not found: {}", target_path.to_str().unwrap());
      }

      let path = format!("{}", target_path.display());

      let code = fs::read_to_string(&path).unwrap();
      let code_file = JavaIdent::parse(&code);
      let first_class = &code_file.classes[0];

      let lines: Vec<String> = code_block.code.split("\n").map(|s| s.to_string()).collect();
      JavaInserter::insert(&path, first_class, lines)
        .expect("TODO: panic message");
    });
  }
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
