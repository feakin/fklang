use fkl_mir::{ContextMap, CustomEnv, Environment};

/// # Custom Function Runner
/// enable load plugins for custom runner
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

/// pre-check
pub async fn kafka_runner(_context: &ContextMap, env: &Environment) {
  let kafka_envs: Vec<CustomEnv> = env.customs.iter()
    .filter(|env| env.name == "kafka")
    .map(|env| env.clone())
    .collect();

  if kafka_envs.len() == 0 {
    panic!("kafka environment is required");
  }

  // Self::execute_kafka(&kafka_envs[0]).await;
}
