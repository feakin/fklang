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
  pub arguments: Vec<VariableDefinition>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Message {
  pub from: String,
  pub to: String,
  pub topic: String,
  pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RpcCall {
  pub from: String,
  pub to: String,
  pub arguments: Vec<VariableDefinition>,
}
