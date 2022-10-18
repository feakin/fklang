use std::collections::HashMap;

use log::info;
use reqwest::blocking::Response;

use fkl_parser::mir::{ContextMap, HttpApiImpl, HttpEndpoint, HttpMethod, Implementation};

pub struct EndpointRunner {
  endpoint: HttpEndpoint,
}

pub(crate) fn execute(context_map: &ContextMap, func_name: &str, impl_name: &str) {
  let mut apis: Vec<HttpApiImpl> = vec![];
  context_map.implementations.iter().for_each(|implementation| {
    if let Implementation::PublishHttpApi(api) = implementation {
      if api.name == impl_name {
        apis.push(api.clone());
      }
    }
  });

  if apis.len() == 0 {
    info!("No implementation found for {}", impl_name);
    return;
  }

  let endpoint = apis[0].endpoint.clone();

  match func_name {
    "request" => {
      let endpoint_runner = EndpointRunner::new(endpoint.clone());
      endpoint_runner.send_request().expect("TODO: panic message");
    }
    _ => {
      info!("No function found for {}", func_name);
    }
  }
}

impl EndpointRunner {
  pub fn new(endpoint: HttpEndpoint) -> Self {
    EndpointRunner {
      endpoint
    }
  }

  pub fn send_request(&self) -> Result<(), ()> {
    let client = reqwest::blocking::Client::new();
    let _req = self.request_to_hashmap();

    let resp: Response;

    match self.endpoint.method {
      HttpMethod::GET => {
        resp = client.get(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::POST => {
        resp = client.post(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::PUT => {
        resp = client.put(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::DELETE => {
        resp = client.delete(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      HttpMethod::PATCH => {
        resp = client.patch(&self.endpoint.path)
          .send()
          .expect("Failed to send request");
      }
      _ => {
        panic!("Unsupported method: {:?}", self.endpoint.method);
      }
    }

    let content_type = resp.headers().get("content-type").unwrap().to_str().unwrap();
    match content_type {
      "application/json" => {
        let json: serde_json::Value = resp.json().expect("Failed to parse response");
        println!("{}", json);
      }
      _ => {
        let text = resp.text().expect("Failed to parse response");
        println!("{}", text);
      }
    }

    Ok(())
  }
  fn request_to_hashmap(&self) -> HashMap<String, String> {
    let map = HashMap::new();
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
      description: "".to_string(),
    };
    let runner = EndpointRunner::new(endpoint);
    let _resp = runner.send_request();
  }

  #[ignore]
  #[test]
  fn http_github() {
    // todo: find a better way
    let endpoint = HttpEndpoint {
      name: "".to_string(),
      method: HttpMethod::GET,
      path: "https://github.com/feakin/".to_string(),
      request: None,
      response: None,
      description: "".to_string(),
    };

    let runner = EndpointRunner::new(endpoint);
    let _resp = runner.send_request();
  }
}
