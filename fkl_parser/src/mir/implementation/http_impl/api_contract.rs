use serde::Deserialize;
use serde::Serialize;
use crate::mir::implementation::validation::Validation;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpEndpoint {
  pub name: String,
  pub description: String,
  pub path: String,
  pub method: HttpMethod,
  pub request: Option<Request>,
  pub response: Option<Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS,
  TRACE,
  CUSTOM(String),
}

impl Default for HttpMethod {
  fn default() -> Self {
    HttpMethod::GET
  }
}

impl HttpMethod {
  pub fn from(method: &str) -> Self {
    match method.to_lowercase().as_str() {
      "get" => HttpMethod::GET,
      "post" => HttpMethod::POST,
      "put" => HttpMethod::PUT,
      "delete" => HttpMethod::DELETE,
      "patch" => HttpMethod::PATCH,
      "head" => HttpMethod::HEAD,
      "options" => HttpMethod::OPTIONS,
      "trace" => HttpMethod::TRACE,
      _ => HttpMethod::CUSTOM(method.to_string()),
    }
  }
}

impl HttpEndpoint {
  pub fn new(name: String) -> Self {
    HttpEndpoint {
      name,
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Request {
  pub name: String,
  pub pre_validate: Option<Validation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Response {
  pub name: String,
  pub post_validate: Option<Validation>,
}

