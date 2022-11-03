use serde::Deserialize;
use serde::Serialize;
use crate::Flow;
use crate::implementation::HttpEndpoint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpApiImpl {
  pub name: String,
  // format: aggregate/entity
  pub target_aggregate: String,
  pub target_entity: String,
  // like "$moduleName:packageName
  pub qualified: String,
  pub endpoint: HttpEndpoint,
  pub flow: Option<Flow>,
}

impl HttpApiImpl {
  pub fn new(name: String) -> Self {
    HttpApiImpl {
      name,
      ..Default::default()
    }
  }

  pub fn target(&self) -> String {
    if !self.target_aggregate.is_empty() {
      return self.target_aggregate.clone();
    }

    if !self.target_entity.is_empty() {
      return self.target_entity.clone();
    }

    return "".to_string();
  }
}


