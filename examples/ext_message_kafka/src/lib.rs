use async_trait::async_trait;

use fkl_ext_api::custom_runner::CustomRunner;
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
    "KafkaRunner"
  }

  async fn execute(&self, _context: &ContextMap, env: &CustomEnv) {
    Self::run_kafka(env).await;
  }
}

impl KafkaExt {
  async fn run_kafka(env: &CustomEnv) {
    let runner = KafkaRunner::from(env);
    runner.send("test", "test").await;
  }
}
