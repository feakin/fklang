use std::fs;
use std::path::PathBuf;
use log::info;
use fkl_codegen_java::gen_http_api;
use fkl_parser::mir::{ContextMap, Implementation};
use fkl_parser::parse;

pub fn run_gen(feakin_path: Option<&PathBuf>, filter_impl: Option<&String>) {
  let mut is_success = false;
  if let Some(path) = feakin_path {
    let feakin = fs::read_to_string(path).unwrap();
    let mir: ContextMap = parse(&feakin).or_else(|e| {
      info!("{}", e);
      Err(e)
    }).unwrap();

    mir.implementations.iter()
      .for_each(|implementation| {
        match implementation {
          Implementation::PublishHttpApi(http) => {
            if let Some(filter_impl) = filter_impl {
              if &http.name == filter_impl {
                let output = gen_http_api(http.clone(), "java");
                info!("{}", output);
              }
            } else {
              let output = gen_http_api(http.clone(), "java");
              info!("{}", output);
            }
          }
          Implementation::PublishEvent => {}
          Implementation::PublishMessage => {}
        }
      });

    is_success = true;
  }

  if !is_success {
    info!("run gen failure!")
  }
}
