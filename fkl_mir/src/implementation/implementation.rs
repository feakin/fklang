use serde::Deserialize;
use serde::Serialize;

use crate::implementation::http_api_impl::HttpApiImpl;
use crate::implementation::Implementation::PublishHttpApi;

// Todo: Subscribe / Publish / Event / Flow

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Implementation {
  PublishHttpApi(HttpApiImpl),
  // todos: add those supports
  PublishEvent,
  // same to PublishEvent ?
  PublishMessage,
}

impl Default for Implementation {
  fn default() -> Self {
    PublishHttpApi(HttpApiImpl::default())
  }
}

impl Implementation {
  pub(crate) fn name(&self) -> String {
    match self {
      PublishHttpApi(impl_) => impl_.name.clone(),
      _ => "".to_string(),
    }
  }
}
