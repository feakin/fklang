use std::collections::HashMap;

use log::info;

use fkl_parser::mir::{HttpEndpoint, HttpMethod};

pub struct EndpointRunner {
  endpoint: HttpEndpoint,
}

impl EndpointRunner {
  pub fn new(endpoint: HttpEndpoint) -> Self {
    EndpointRunner {
      endpoint
    }
  }

  pub fn send_request(&self) -> Result<(), ()> {
    let client = reqwest::blocking::Client::new();
    let req = self.request_to_hashmap();

    match self.endpoint.method {
      HttpMethod::GET => {
        client.get(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::POST => {
        client.post(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::PUT => {
        client.put(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::DELETE => {
        client.delete(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::PATCH => {
        client.patch(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      _ => {
        info!("Unsupported method: {:?}", self.endpoint.method);
      }
    }

    Ok(())
  }
  fn request_to_hashmap(&self) -> HashMap<String, String> {
    let mut map = HashMap::new();
    // todo: convert request
    map
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn test_request_to_hashmap() {
    let endpoint = HttpEndpoint {
      name: "".to_string(),
      method: HttpMethod::GET,
      path: "/test".to_string(),
      request: None,
      response: None,
      description: "".to_string()
    };
    let runner = EndpointRunner::new(endpoint);
    let map = runner.send_request();
  }
}
