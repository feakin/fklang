use std::time::Duration;

use log::{info, trace};
use rdkafka::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

pub struct KafkaConnector {
  pub host: String,
  pub port: u16,
}

impl KafkaConnector {
  pub fn new(host: &str, port: u16) -> Self {
    KafkaConnector {
      host: host.to_string(),
      port,
    }
  }
}

impl KafkaConnector {
  pub async fn send(&self, topic: &str, message: &str) {
    let brokers = format!("{}:{}", self.host, self.port);

    let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", &brokers)
      .set("message.timeout.ms", "5000")
      .create()
      .expect("Producer creation error");

    trace!("Producer created");

    let delivery_status = producer
      .send(
        FutureRecord::to(topic)
          .payload(&format!("Message {}", message))
          .key("Key")
          .headers(OwnedHeaders::new()),
        Duration::from_secs(0),
      )
      .await;

    info!("Delivery status: {:?}", delivery_status);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  #[ignore]
  async fn test_send() {
    let connector = KafkaConnector::new("localhost", 9092);
    connector.send("test", "test").await;
  }
}

