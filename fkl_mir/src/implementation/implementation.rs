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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_implementation_name() {
    let impl_ = Implementation::default();
    assert_eq!(impl_.name(), "");
  }

  #[test]
  fn http_api_name() {
    let mut api_impl = HttpApiImpl::default();
    api_impl.name = "test".to_string();
    let implementation = PublishHttpApi(api_impl);
    assert_eq!(implementation.name(), "test");
  }
}
