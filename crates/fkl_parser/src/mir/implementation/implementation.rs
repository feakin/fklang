use serde::Deserialize;
use serde::Serialize;

use crate::mir::implementation::http_api_impl::HttpApiImpl;
use crate::mir::implementation::Implementation::PublishHttpApi;

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
