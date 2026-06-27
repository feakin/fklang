use std::time::Duration;

use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;

use fkl_ext_api::custom_runner::Argument;
use fkl_mir::CustomEnv;

pub struct KafkaRunner {
  pub host: String,
  pub port: u16,
}

impl KafkaRunner {
  pub fn from(env: &CustomEnv) -> KafkaRunner {
    let port = env
      .attrs
      .iter()
      .find(|it| it.name == "port")
      .and_then(|env| env.initializer.as_deref())
      .and_then(|value| value.parse::<u16>().ok())
      .unwrap_or(9092);

    let host = env
      .attrs
      .iter()
      .find(|it| it.name == "host")
      .and_then(|env| env.initializer.as_deref())
      .unwrap_or("localhost");

    KafkaRunner {
      host: host.to_string(),
      port,
    }
  }

  pub fn from_args(args: &[Argument]) -> KafkaRunner {
    let host = args
      .iter()
      .find(|arg| arg.name == "host")
      .map(|arg| arg.value.as_str())
      .unwrap_or("localhost");
    let port = args
      .iter()
      .find(|arg| arg.name == "port")
      .and_then(|arg| arg.value.parse::<u16>().ok())
      .unwrap_or(9092);

    KafkaRunner {
      host: host.to_string(),
      port,
    }
  }

  pub fn brokers(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }

  pub async fn send(&self, topic: &str, message: &str) {
    let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", &self.brokers())
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
