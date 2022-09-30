use serde::Deserialize;
use serde::Serialize;
use crate::mir::implementation::validation::Validation;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpEndpoint {
  pub name: String,
  pub description: String,
  pub path: String,
  pub method: String,
  pub request: Option<Request>,
  pub response: Option<Response>,
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
  pre_validate: Option<Validation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Response {
  pub name: String,
  post_validate: Option<Validation>,
}
