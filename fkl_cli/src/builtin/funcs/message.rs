use fkl_parser::mir::{ContextMap, CustomEnv, Environment};

use crate::message::kafka_connector::KafkaConnector;

pub async fn kafka_runner(_context: &ContextMap, env: &Environment) {
  let kafka_envs: Vec<CustomEnv> = env.customs.iter()
    .filter(|env| env.name == "kafka")
    .map(|env| env.clone())
    .collect();

  if kafka_envs.len() == 0 {
    panic!("kafka environment is required");
  }

  execute_kafka(&kafka_envs[0]).await;
}

pub async fn execute_kafka(env: &CustomEnv) {
  let port: u16 = match env.attrs.iter().filter(|it| it.name == "port").next() {
    None => { 9092 }
    Some(env) => {
      env.initializer.as_ref().unwrap().as_str().parse::<u16>().unwrap()
    }
  };

  let host = match env.attrs.iter().filter(|it| it.name == "host").next() {
    None => { "localhost" }
    Some(env) => {
      env.initializer.as_ref().unwrap().as_str()
    }
  };

  let connector = KafkaConnector::new(host, port);
  connector.send("test", "demo").await;
}
