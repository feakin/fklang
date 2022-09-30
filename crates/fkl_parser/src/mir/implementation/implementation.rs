use crate::mir::implementation::http_api_impl::HttpApiImpl;

// Subscribe / Publish / Event / Flow

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Implementation {
  PublishHttpApi(HttpApiImpl),
  // todos: add those supports
  PublishEvent,
  // same to PublishEvent ?
  PublishMessage,
}
