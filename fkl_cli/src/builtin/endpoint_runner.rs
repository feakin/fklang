use std::collections::HashMap;

use log::info;
use reqwest::blocking::Response;
use reqwest::header;

use fkl_parser::mir::{ContextMap, HttpApiImpl, HttpEndpoint, HttpMethod, Implementation};
use fkl_parser::mir::authorization::HttpAuthorization;
use crate::RunFuncName;

pub struct EndpointRunner {
  endpoint: HttpEndpoint,
}

pub(crate) fn execute(context_map: &ContextMap, func_name: &RunFuncName, impl_name: &str) {
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
    RunFuncName::HttpRequest => {
      let endpoint_runner = EndpointRunner::new(endpoint.clone());
      endpoint_runner.send_request().expect("TODO: panic message");
    }
    RunFuncName::MockServer => {
      // TODO: implement mock server
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
    let mut headers = header::HeaderMap::new();
    if let Some(http_auth) = &self.endpoint.auth {
      match http_auth {
        HttpAuthorization::Basic(username, password) => {
          let header = format!("Basic {}", base64::encode(format!("{}:{}", username, password)));
          headers.insert(header::AUTHORIZATION, header.parse().unwrap());
        }
        HttpAuthorization::Bearer(token) => {
          let header = format!("Bearer {}", token);
          headers.insert(header::AUTHORIZATION, header.parse().unwrap());
        }
        HttpAuthorization::Digest(username, password) => {
          let header = format!("Digest {}", base64::encode(format!("{}:{}", username, password)));
          headers.insert(header::AUTHORIZATION, header.parse().unwrap());
        }
        HttpAuthorization::None => {}
      }
    }

    info!("headers: {:?}", headers.clone());


    let _req = self.request_to_hashmap();
    let client = reqwest::blocking::Client::builder()
      .default_headers(headers)
      .build()
      .expect("TODO: panic message");

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
      auth: None,
    };
    let runner = EndpointRunner::new(endpoint);
    let _resp = runner.send_request();
  }

  #[ignore]
  #[test]
  fn http_github() {
    let endpoint = HttpEndpoint {
      name: "".to_string(),
      method: HttpMethod::GET,
      path: "https://github.com/feakin/".to_string(),
      request: None,
      response: None,
      description: "".to_string(),
      auth: None,
    };

    let runner = EndpointRunner::new(endpoint);
    let _resp = runner.send_request();
  }
}
