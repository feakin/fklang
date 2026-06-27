use async_trait::async_trait;
use fkl_ext_api::custom_runner::{Argument, CustomRunner};
use fkl_mir::{ContextMap, CustomEnv};

pub struct AwsCodegenRunner;

#[async_trait]
impl CustomRunner for AwsCodegenRunner {
  fn name(&self) -> &str {
    "aws-codegen"
  }

  async fn execute(&self, _context: &ContextMap, _env: &CustomEnv) {}

  async fn send_command(&self, command: &str, args: &[Argument]) -> Option<String> {
    if command != "lambda-handler" {
      return None;
    }

    Some(generate_lambda_handler(args))
  }

  fn list_commands(&self) -> Vec<String> {
    vec!["lambda-handler".to_string()]
  }
}

#[no_mangle]
pub unsafe fn _fkl_create_runner() -> *mut dyn CustomRunner {
  Box::into_raw(Box::new(AwsCodegenRunner))
}

fn generate_lambda_handler(args: &[Argument]) -> String {
  let package = arg_value(args, "package").unwrap_or("com.example");
  let handler = arg_value(args, "handler").unwrap_or("FklLambdaHandler");

  format!(
    r#"package {package};

public class {handler} implements com.amazonaws.services.lambda.runtime.RequestHandler<String, String> {{
  @Override
  public String handleRequest(String input, com.amazonaws.services.lambda.runtime.Context context) {{
    return input;
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
  fn exposes_aws_codegen_runner_name() {
    assert_eq!(AwsCodegenRunner.name(), "aws-codegen");
  }

  #[test]
  fn lists_lambda_handler_command() {
    assert_eq!(
      AwsCodegenRunner.list_commands(),
      vec!["lambda-handler".to_string()]
    );
  }

  #[tokio::test]
  async fn generates_java_lambda_handler_from_arguments() {
    let result = AwsCodegenRunner
      .send_command(
        "lambda-handler",
        &[
          Argument {
            name: "package".to_string(),
            value: "com.example.orders".to_string(),
          },
          Argument {
            name: "handler".to_string(),
            value: "OrderCreatedHandler".to_string(),
          },
        ],
      )
      .await;

    assert_eq!(
      result,
      Some(
        r#"package com.example.orders;

public class OrderCreatedHandler implements com.amazonaws.services.lambda.runtime.RequestHandler<String, String> {
  @Override
  public String handleRequest(String input, com.amazonaws.services.lambda.runtime.Context context) {
    return input;
  }
}
"#
        .to_string()
      )
    );
  }

  #[tokio::test]
  async fn ignores_unknown_commands() {
    let result = AwsCodegenRunner
      .send_command(
        "cloudformation",
        &[Argument {
          name: "name".to_string(),
          value: "orders".to_string(),
        }],
      )
      .await;

    assert_eq!(result, None);
  }
}
