use fkl_codegen_java::gen_http_api;
use fkl_parser::mir::{ContextMap, Implementation};
use std::path::PathBuf;
use std::fs;
use log::error;
use fkl_parser::parse;

pub struct CodeBlock {
  pub target_layer: DddLayer,
  pub code: String,
}

pub enum DddLayer {
  Interface,
  Application,
  Domain,
  Infrastructure,
}

pub fn code_gen_exec(feakin_path: Option<&PathBuf>, filter_impl: Option<&String>) {
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
  let mut has_layered_define = mir.layered.is_some();
  if has_layered_define {
    // let layer_map = LayerMap::from(mir.layered.clone().unwrap());
    // JavaInserter::insert(&layer_map, code_blocks).expect("TODO: panic message");
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
              let output = gen_http_api(http.clone(), "java");
              codes.push(CodeBlock {
                target_layer: DddLayer::Interface,
                code: output,
              });
            }
          } else {
            let output = gen_http_api(http.clone(), "java");
            codes.push(CodeBlock {
              target_layer: DddLayer::Interface,
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
