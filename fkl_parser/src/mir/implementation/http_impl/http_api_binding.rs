use serde::Deserialize;
use serde::Serialize;

use crate::mir::implementation::http_impl::HttpEndpoint;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpApiBinding {
  pub name: String,
  pub defined: Option<HttpApiDefine>,
  pub api_contract: Option<HttpEndpoint>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct HttpApiDefine {

}

