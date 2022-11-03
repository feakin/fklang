pub mod spring_gen;
pub mod nlp;
pub mod naming;
pub mod comments;

pub use spring_gen::entity_gen::*;
pub use spring_gen::jpa_gen::*;
pub use spring_gen::spring_code_gen::*;

use fkl_mir::implementation::HttpApiImpl;

pub struct GenCode {
  pub code: String,
  pub method_name: String,
}

pub fn gen_http_api(api: &HttpApiImpl, _framework: &str) -> GenCode {
  let mut endpoint = api.endpoint.clone();
  endpoint.name = api.name.clone();

  let spring_code_gen = SpringCodeGen::from(&endpoint, &api.flow);
  let annotation = spring_code_gen.method_annotation;
  let method_header = spring_code_gen.method_header;
  let method_name = spring_code_gen.method_name;
  let ai_comments = spring_code_gen.ai_comments
    .iter()
    .map(|comment| format!("        {}", comment))
    .collect::<Vec<String>>()
    .join("\n");

  let code = format!(r#"
    {}
    {} {{
{}
    }}
"#, annotation, method_header, ai_comments);

  GenCode {
    code,
    method_name,
  }
}

#[cfg(test)]
mod tests {
  use fkl_mir::implementation::{HttpApiImpl, HttpEndpoint};

  use crate::gen_http_api;

  #[test]
  fn basic_convert() {
    let mut api_impl = HttpApiImpl::default();
    api_impl.qualified = "com.feakin.demo".to_string();
    api_impl.endpoint = HttpEndpoint::default();

    let output = gen_http_api(&api_impl, "spring");
    assert_eq!(output.code, "\n    @GetMapping\n    public void main() {\n\n    }\n");
    assert_eq!(output.method_name, "main");
  }
}
