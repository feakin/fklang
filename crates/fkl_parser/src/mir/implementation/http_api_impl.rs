use serde::Deserialize;
use serde::Serialize;
use crate::mir::implementation::http_impl::HttpApiBinding;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpApiImpl {
  pub name: String,
  // format: aggregate/entity
  pub target_aggregate: String,
  pub target_entity: String,
  // like "$moduleName:packageName
  pub qualified: String,
  pub api_binding: Vec<HttpApiBinding>,
}

