use std::fmt::Display;
use serde::Deserialize;
use serde::Serialize;
use crate::mir::binding::VariableDefinition;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Step {
  MethodCall(MethodCall),
  Message(Message),
  RpcCall(RpcCall),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MethodCall {
  pub name: String,
  pub object: String,
  pub method: String,
  pub parameters: Vec<VariableDefinition>,
  pub return_type: Option<VariableDefinition>,
}

impl MethodCall {
  pub fn new(name: String) -> Self {
    MethodCall {
      name,
      ..Default::default()
    }
  }
}

impl Display for MethodCall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let return_type_str: String = if let Some(return_type) = &self.return_type {
      format!("{}:{}", return_type.name, return_type.type_type)
    } else {
      "".to_owned()
    };
    let source = format!("{}.{}", &self.object, &self.method);
    let params = &self.parameters.iter().map(|p| format!("{}:{}", p.name, p.type_type)).collect::<Vec<String>>().join(", ");

    if return_type_str.is_empty() {
      write!(f, "call {} with ({})", source, params)
    } else {
      write!(f, "get {} from {} with ({})", return_type_str, source, params)
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Message {
  pub from: String,
  pub to: String,
  pub topic: String,
  pub message: String,
}

impl Display for Message {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "send {} from {} to {}", self.message, self.from, self.topic)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RpcCall {
  pub from: String,
  pub to: String,
  pub arguments: Vec<VariableDefinition>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_call() {
    let call = MethodCall {
      name: "call".to_owned(),
      object: "object".to_owned(),
      method: "method".to_owned(),
      parameters: vec![
        VariableDefinition {
          name: "param1".to_owned(),
          type_type: "type1".to_owned(),
          initializer: None
        },
        VariableDefinition {
          name: "param2".to_owned(),
          type_type: "type2".to_owned(),
          initializer: None
        },
      ],
      return_type: Some(VariableDefinition {
        name: "return".to_owned(),
        type_type: "type3".to_owned(),
        initializer: None
      }),
    };
    let comment = call.to_string();
    assert_eq!(comment, "get return:type3 from object.method with (param1:type1, param2:type2)");
  }

  #[test]
  fn test_method_call_without_return() {
    let call = MethodCall {
      name: "call".to_owned(),
      object: "object".to_owned(),
      method: "method".to_owned(),
      parameters: vec![
        VariableDefinition {
          name: "param1".to_owned(),
          type_type: "type1".to_owned(),
          initializer: None
        },
        VariableDefinition {
          name: "param2".to_owned(),
          type_type: "type2".to_owned(),
          initializer: None
        },
      ],
      return_type: None,
    };
    let comment = call.to_string();
    assert_eq!(comment, "call object.method with (param1:type1, param2:type2)");
  }

  #[test]
  fn format_message() {
    let message = Message {
      from: "object".to_owned(),
      to: "to".to_owned(),
      topic: "event:event".to_owned(),
      message: "content".to_owned(),
    };
    let comment = message.to_string();
    assert_eq!(comment, "send content from object to event:event");
  }
}
