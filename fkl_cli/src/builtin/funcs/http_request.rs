use log::info;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use reqwest::header::HeaderMap;

use fkl_mir::{ContextMap, HttpApiImpl, HttpEndpoint, HttpMethod, Implementation, Struct};
use fkl_mir::authorization::HttpAuthorization;

use crate::highlighter::Highlighter;
use crate::mock::fake_user_agent::UserAgent;
use crate::mock::fake_value::FakeValue;
use crate::RunFuncName;

pub struct EndpointRunner {
  endpoint: HttpEndpoint,
  request_struct: Option<Struct>,
}

pub(crate) fn endpoint_runner(context_map: &ContextMap, func_name: &RunFuncName, impl_name: &str) {
  let mut apis: Vec<HttpApiImpl> = vec![];

  let _ = &context_map.implementations.iter().for_each(|implementation| {
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

  let endpoint = &apis[0].endpoint;

  match func_name {
    RunFuncName::HttpRequest => {
      let mut runner = EndpointRunner::new(endpoint.clone());

      if let Some(req) = &endpoint.request {
        if let Some(struct_) = &context_map.get_struct(&req.name) {
          runner.request_struct = Some(struct_.clone());
        }
      }

      runner.send_request().expect("TODO: panic message");
    }
    _ => {}
  }
}

impl EndpointRunner {
  pub fn new(endpoint: HttpEndpoint) -> Self {
    EndpointRunner {
      endpoint,
      request_struct: None,
    }
  }

  pub fn send_request(&self) -> Result<(), ()> {
    let mut headers = self.headers();
    headers.insert(header::USER_AGENT, UserAgent::random().parse().unwrap());

    info!("headers: {:?}", headers.clone());

    let body = self.create_request_body();
    let resp = self.do_request(headers, body);

    self.handle_response(resp);

    Ok(())
  }

  fn handle_response(&self, resp: Response) {
    let content_type = resp.headers().get("content-type").unwrap().to_str().unwrap();
    info!("Content-Type: {}", content_type);
    match content_type {
      "application/json" => {
        let json: serde_json::Value = resp.json().expect("Failed to parse response");
        Highlighter::json(&json.to_string());
      }
      "text/plain; charset=utf-8" => {
        let text = resp.text().unwrap();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
          Highlighter::json(&serde_json::to_string_pretty(&json).unwrap());
        } else {
          println!("{}", text);
        }
      }
      _ => {
        let text = resp.text().expect("Failed to parse response");
        println!("{}", text);
      }
    }
  }

  fn do_request(&self, headers: HeaderMap, request: Option<String>) -> Response {
    let client = Client::builder()
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
        if let Some(req) = &request {
          info!("body: {:?}", req.clone());
        }

        resp = client.post(&self.endpoint.path)
          .body(request.unwrap_or("".to_string()))
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
    resp
  }

  fn headers(&self) -> HeaderMap {
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

    headers
  }

  fn create_request_body(&self) -> Option<String> {
    return if let Some(req) = &self.request_struct {
      let map = FakeValue::fake(&req.fields);
      Some(serde_json::to_string(&map).unwrap())
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use fkl_mir::{Field, Request};
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

  #[test]
  #[should_panic]
  fn http_github() {
    let endpoint = HttpEndpoint {
      name: "".to_string(),
      method: HttpMethod::POST,
      path: "/sample".to_string(),
      request: Some(Request {
        name: "User".to_string(),
        pre_validate: None
      }),
      response: None,
      description: "".to_string(),
      auth: None,
    };

    let mut runner = EndpointRunner::new(endpoint);
    runner.request_struct = Some(Struct {
      name: "User".to_string(),
      fields: vec![Field {
        name: "name".to_string(),
        initializer: None,
        type_type: "String".to_string(),
      }],
    });

    let _resp = runner.send_request();
  }
}
