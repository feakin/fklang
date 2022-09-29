use serde::Deserialize;
use serde::Serialize;
use crate::mir::implementation::validation::Validation;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Endpoint {
  pub name: String,
  pub description: String,
  pub path: String,
  pub method: String,
  pub request: Option<Request>,
  pub response: Option<Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Request {
  pre_validate: Option<Validation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Response {
  post_validate: Option<Validation>,
}
