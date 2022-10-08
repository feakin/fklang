use genco::fmt;
use genco::prelude::*;

pub mod spring_gen;
pub mod nlp;

pub use spring_gen::entity_gen::*;
pub use spring_gen::jpa_gen::*;
pub use spring_gen::spring_code_gen::*;

use fkl_parser::mir::implementation::HttpApiImpl;

pub fn gen_http_api(api: HttpApiImpl, _framework: &str) -> anyhow::Result<String> {
  let mut w = fmt::FmtWriter::new(String::new());

  let fmt = fmt::Config::from_lang::<Java>().with_newline("\n");
  // package from default impl
  let mut config: java::Config = java::Config::default();
  if !api.qualified.is_empty() {
    config = java::Config::default().with_package(api.qualified);
  }

  let mut endpoint = api.endpoint.clone();
  endpoint.name = api.name.clone();
  let spring_code_gen = SpringCodeGen::from(endpoint);

  let annotation = spring_code_gen.method_annotation;
  let newline = "\n";
  let method_header = spring_code_gen.method_header;

  let flows = spring_code_gen.ai_comments;

  let tokens: java::Tokens = quote! {
        $annotation$newline$method_header {
            $flows
        }
    };

  tokens.format_file(&mut w.as_formatter(&fmt), &config).unwrap();

  Ok(w.into_inner())
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::implementation::{HttpApiImpl, HttpEndpoint};

  use crate::gen_http_api;

  #[test]
  #[ignore]
  fn basic_convert() {
    let mut api_impl = HttpApiImpl::default();
    api_impl.qualified = "com.feakin.demo".to_string();
    api_impl.endpoint = HttpEndpoint::default();

    let output = gen_http_api(api_impl, "spring").unwrap();
    assert_eq!(output, r#"package com.feakin.demo;

@GetMapping
public void main() { }
"#)
  }
}
