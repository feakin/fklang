use serde::Deserialize;
use serde::Serialize;

/// Http Authorization, used for http request, support basic and bearer, digest, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpAuthorization {
  None,
  Basic(String, String),
  Digest(String, String),
  Bearer(String),
}

impl Default for HttpAuthorization {
  fn default() -> Self {
    HttpAuthorization::None
  }
}

impl HttpAuthorization {
  pub fn from(auth_type: &str, username: Option<String>, password: Option<String>) -> Self {
    match auth_type.to_lowercase().as_str() {
      "basic" => {
        if let Some(username) = username {
          if let Some(password) = password {
            return HttpAuthorization::Basic(username, password);
          }
        }
        HttpAuthorization::None
      }
      "digest" => {
        if let Some(username) = username {
          if let Some(password) = password {
            return HttpAuthorization::Digest(username, password);
          }
        }
        HttpAuthorization::None
      }
      "bearer" => {
        if let Some(token) = username {
          return HttpAuthorization::Bearer(token);
        }
        HttpAuthorization::None
      }
      _ => HttpAuthorization::None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from() {
    assert_eq!(
      HttpAuthorization::from("basic", Some("username".to_string()), Some("password".to_string())),
      HttpAuthorization::Basic("username".to_string(), "password".to_string())
    );
    assert_eq!(
      HttpAuthorization::from("digest", Some("username".to_string()), Some("password".to_string())),
      HttpAuthorization::Digest("username".to_string(), "password".to_string())
    );
    assert_eq!(
      HttpAuthorization::from("bearer", Some("token".to_string()), None),
      HttpAuthorization::Bearer("token".to_string())
    );
    assert_eq!(
      HttpAuthorization::from("unknown", Some("username".to_string()), Some("password".to_string())),
      HttpAuthorization::None
    );
  }
}
