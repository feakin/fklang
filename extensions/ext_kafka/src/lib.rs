use async_trait::async_trait;

use fkl_ext_api::custom_runner::{Argument, CustomRunner};
use fkl_mir::{ContextMap, CustomEnv};
use kafka_runner::KafkaRunner;

mod kafka_runner;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _fkl_create_runner() -> *mut dyn CustomRunner {
  let object = KafkaExt {};
  let boxed = Box::new(object);
  Box::into_raw(boxed)
}

pub struct KafkaExt {}

#[async_trait]
impl CustomRunner for KafkaExt {
  fn name(&self) -> &str {
    "kafka"
  }

  async fn execute(&self, _context: &ContextMap, env: &CustomEnv) {
    Self::run_kafka(env).await;
  }

  async fn send_command(&self, command: &str, args: &[Argument]) -> Option<String> {
    if command != "broker" {
      return None;
    }

    Some(KafkaRunner::from_args(args).brokers())
  }

  fn list_commands(&self) -> Vec<String> {
    vec!["broker".to_string()]
  }
}

impl KafkaExt {
  async fn run_kafka(env: &CustomEnv) {
    let runner = KafkaRunner::from(env);
    runner.send("test", "test").await;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use fkl_ext_api::custom_runner::Argument;
  use fkl_mir::VariableDefinition;

  fn attr(name: &str, value: &str) -> VariableDefinition {
    VariableDefinition {
      name: name.to_string(),
      type_type: "String".to_string(),
      initializer: Some(value.to_string()),
    }
  }

  #[test]
  fn exposes_kafka_runner_name() {
    assert_eq!(KafkaExt {}.name(), "kafka");
  }

  #[test]
  fn lists_broker_command() {
    assert_eq!(KafkaExt {}.list_commands(), vec!["broker".to_string()]);
  }

  #[tokio::test]
  async fn returns_default_broker_without_connecting_to_kafka() {
    let result = KafkaExt {}.send_command("broker", &[]).await;
    assert_eq!(result, Some("localhost:9092".to_string()));
  }

  #[tokio::test]
  async fn returns_argument_broker_without_connecting_to_kafka() {
    let result = KafkaExt {}
      .send_command(
        "broker",
        &[
          Argument {
            name: "host".to_string(),
            value: "broker.internal".to_string(),
          },
          Argument {
            name: "port".to_string(),
            value: "19092".to_string(),
          },
        ],
      )
      .await;

    assert_eq!(result, Some("broker.internal:19092".to_string()));
  }

  #[test]
  fn parses_broker_from_environment_attrs() {
    let env = CustomEnv {
      name: "kafka".to_string(),
      attrs: vec![attr("host", "broker.internal"), attr("port", "19092")],
    };

    let runner = KafkaRunner::from(&env);
    assert_eq!(runner.host, "broker.internal");
    assert_eq!(runner.port, 19092);
  }

  #[test]
  fn falls_back_to_default_port_for_invalid_environment_attr() {
    let env = CustomEnv {
      name: "kafka".to_string(),
      attrs: vec![
        attr("host", "broker.internal"),
        attr("port", "not-a-number"),
      ],
    };

    let runner = KafkaRunner::from(&env);
    assert_eq!(runner.host, "broker.internal");
    assert_eq!(runner.port, 9092);
  }

  #[tokio::test]
  async fn ignores_unknown_commands() {
    let result = KafkaExt {}
      .send_command(
        "publish",
        &[Argument {
          name: "topic".to_string(),
          value: "orders".to_string(),
        }],
      )
      .await;

    assert_eq!(result, None);
  }
}
