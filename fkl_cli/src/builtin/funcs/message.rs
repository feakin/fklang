use fkl_parser::mir::{CustomEnv, Field};

use crate::message::kafka_connector::KafkaConnector;

pub fn message_queue_runner(env: CustomEnv) {

}

pub fn is_kafka_config(env: &CustomEnv) -> bool {
  if env.name != "kafka" {
    return false;
  }

  return true;
}

// pub fn kafka_runner(env: CustomConfig) {
//   let port =match env.attrs.iter().filter(|it| it.name == "port").next() {
//     None => { 9092 }
//     Some(env) => {
//       return env.name
//     }
//   }
// }
