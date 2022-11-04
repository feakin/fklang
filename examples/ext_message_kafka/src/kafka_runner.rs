use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

use fkl_mir::CustomEnv;

pub struct KafkaRunner {
  pub host: String,
  pub port: u16,
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
