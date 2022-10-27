use std::collections::HashMap;

use log::info;
use reqwest::blocking::{Client, Response};
use reqwest::header;
use reqwest::header::HeaderMap;

use fkl_parser::mir::{ContextMap, HttpApiImpl, HttpEndpoint, HttpMethod, Implementation};
use fkl_parser::mir::authorization::HttpAuthorization;
use crate::cli_ctx::CliCtx;

use crate::RunFuncName;

pub struct EndpointRunner<'c> {
  endpoint: HttpEndpoint,
  ctx: &'c CliCtx,
}

pub(crate) fn execute(context_map: &ContextMap, func_name: &RunFuncName, impl_name: &str, ctx: &CliCtx) {
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
      let runner = EndpointRunner::new(endpoint, ctx);
      runner.send_request().expect("TODO: panic message");
    }
    RunFuncName::Guarding => {}
  }
}

impl<'c> EndpointRunner<'c> {
  pub fn new(endpoint: HttpEndpoint, ctx: &'c CliCtx) -> Self {
    EndpointRunner {
      endpoint,
      ctx,
    }
  }

  pub fn send_request(&self) -> Result<(), ()> {
    let headers = self.headers();

    let body = self.request_to_hashmap();
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
        self.ctx.highlighter.json(&json.to_string());
      }
      "text/plain; charset=utf-8" => {
        let text = resp.text().unwrap();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
          self.ctx.highlighter.json(&serde_json::to_string_pretty(&json).unwrap());
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

  fn do_request(&self, headers: HeaderMap, _req: HashMap<String, String>) -> Response {
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

    info!("headers: {:?}", headers.clone());
    headers
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
    let ctx = CliCtx::new();
    let runner = EndpointRunner::new(endpoint, &ctx);
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

    let ctx = CliCtx::new();
    let runner = EndpointRunner::new(endpoint, &ctx);
    let _resp = runner.send_request();
  }
}
