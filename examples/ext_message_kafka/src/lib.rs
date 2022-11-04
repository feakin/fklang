use std::time::Duration;

use async_trait::async_trait;
use rdkafka::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

use fkl_ext_api::custom_runner::CustomRunner;
use fkl_mir::{ContextMap, CustomEnv, Environment};

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _fkl_create_runner() -> *mut dyn CustomRunner {
  let object = KafkaExt {};
  let boxed = Box::new(object);
  Box::into_raw(boxed)
}

pub struct KafkaExt {}

pub struct KafkaRunner {
  pub host: String,
  pub port: u16,
}

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
  #[allow(dead_code)]
  async fn run_kafka(env: &CustomEnv) {
    let runner = KafkaRunner::from(env);
    runner.send("test", "test").await;
  }
}

impl KafkaRunner {
  pub fn from(env: &CustomEnv) -> KafkaRunner {
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

    KafkaRunner { host: host.to_string(), port }
  }

  pub async fn send(&self, topic: &str, message: &str) {
    let brokers = format!("{}:{}", self.host, self.port);

    let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", &brokers)
      .set("message.timeout.ms", "5000")
      .create()
      .expect("Producer creation error");

    let delivery_status = producer
      .send(
        FutureRecord::to(topic)
          .payload(&format!("Message {}", message))
          .key("Key")
          .headers(OwnedHeaders::new()),
        Duration::from_secs(0),
      )
      .await;

    println!("Delivery status: {:?}", delivery_status);
  }
}
