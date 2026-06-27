use async_trait::async_trait;
use fkl_ext_api::custom_runner::{Argument, CustomRunner};
use fkl_mir::{ContextMap, CustomEnv};

pub struct SwaggerSourceSetRunner;

#[async_trait]
impl CustomRunner for SwaggerSourceSetRunner {
  fn name(&self) -> &str {
    "sourceset-swagger"
  }

  async fn execute(&self, _context: &ContextMap, _env: &CustomEnv) {}

  async fn send_command(&self, command: &str, args: &[Argument]) -> Option<String> {
    if command != "source-set" {
      return None;
    }

    Some(generate_source_set(args))
  }

  fn list_commands(&self) -> Vec<String> {
    vec!["source-set".to_string()]
  }
}

#[no_mangle]
pub unsafe fn _fkl_create_runner() -> *mut dyn CustomRunner {
  Box::into_raw(Box::new(SwaggerSourceSetRunner))
}

fn generate_source_set(args: &[Argument]) -> String {
  let collection = arg_value(args, "collection").unwrap_or("sourceSet");
  let name = arg_value(args, "name").unwrap_or("swagger");
  let src_dir = arg_value(args, "srcDir").unwrap_or("src/main/resources/swagger");

  format!(
    r#"SourceSet {collection} {{
  {name} {{
    parser: "Swagger"
    extension: "json"
    srcDir: ["{src_dir}"]
  }}
}}
"#
  )
}

fn arg_value<'a>(args: &'a [Argument], name: &str) -> Option<&'a str> {
  args
    .iter()
    .find(|argument| argument.name == name)
    .map(|argument| argument.value.as_str())
}

#[cfg(test)]
mod tests {
  use super::*;
  use fkl_ext_api::custom_runner::{Argument, CustomRunner};

  #[test]
  fn exposes_swagger_sourceset_runner_name() {
    assert_eq!(SwaggerSourceSetRunner.name(), "sourceset-swagger");
  }

  #[test]
  fn lists_source_set_command() {
    assert_eq!(
      SwaggerSourceSetRunner.list_commands(),
      vec!["source-set".to_string()]
    );
  }

  #[tokio::test]
  async fn generates_swagger_source_set_from_arguments() {
    let result = SwaggerSourceSetRunner
      .send_command(
        "source-set",
        &[
          Argument {
            name: "collection".to_string(),
            value: "apiSources".to_string(),
          },
          Argument {
            name: "name".to_string(),
            value: "ordersApi".to_string(),
          },
          Argument {
            name: "srcDir".to_string(),
            value: "api/openapi".to_string(),
          },
        ],
      )
      .await;

    assert_eq!(
      result,
      Some(
        r#"SourceSet apiSources {
  ordersApi {
    parser: "Swagger"
    extension: "json"
    srcDir: ["api/openapi"]
  }
}
"#
        .to_string()
      )
    );
  }

  #[tokio::test]
  async fn generated_source_set_is_parseable_fkl() {
    let source = SwaggerSourceSetRunner
      .send_command("source-set", &[])
      .await
      .expect("source-set command");

    fkl_parser::parse(&source).expect("generated source set should parse");
  }

  #[tokio::test]
  async fn ignores_unknown_commands() {
    let result = SwaggerSourceSetRunner
      .send_command(
        "parse",
        &[Argument {
          name: "srcDir".to_string(),
          value: "api/openapi".to_string(),
        }],
      )
      .await;

    assert_eq!(result, None);
  }
}
