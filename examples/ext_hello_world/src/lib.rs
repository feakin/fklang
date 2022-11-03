use std::time::Duration;

use async_trait::async_trait;
use rdkafka::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

use fkl_ext_api::custom_runner::CustomRunner;
use fkl_mir::{ContextMap, CustomEnv};

pub struct KafkaRunner {}

pub struct KafkaConfig {
  pub host: String,
  pub port: u16,
}

#[async_trait]
impl CustomRunner for KafkaRunner {
  fn name(&self) -> &str {
    "KafkaRunner"
  }

  async fn execute(&self, _context: &ContextMap, _env: &CustomEnv) {
    let config = KafkaConfig {
      host: "localhost".to_string(),
      port: 9092,
    };

    send("test", "test", config).await;
    println!("KafkaRunner");
  }
}

pub async fn send(topic: &str, message: &str, config: KafkaConfig) {
  let brokers = format!("{}:{}", config.host, config.port);

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
