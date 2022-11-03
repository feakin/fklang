use fkl_mir::{ContextMap, Environment};

use crate::builtin::funcs::message::kafka_runner;

pub async fn custom_function_runner(context_map: &ContextMap, env: &Environment, fun_name: &str) {
  match fun_name {
    "kafka" => {
      kafka_runner(context_map, env).await;
    }
    _ => {
      panic!("cannot find function: {}", fun_name);
    }
  }
}
