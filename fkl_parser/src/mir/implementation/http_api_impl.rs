use serde::Deserialize;
use serde::Serialize;
use crate::mir::Flow;
use crate::mir::implementation::HttpEndpoint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpApiImpl {
  pub name: String,
  // format: aggregate/entity
  pub target_aggregate: String,
  pub target_entity: String,
  // like "$moduleName:packageName
  pub qualified: String,
  pub endpoint: HttpEndpoint,
  pub flows: Vec<Flow>,
}

impl HttpApiImpl {
  pub fn new(name: String) -> Self {
    HttpApiImpl {
      name,
      ..Default::default()
    }
  }
}


