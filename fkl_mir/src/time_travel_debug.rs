use serde::Deserialize;
use serde::Serialize;

use crate::{HttpApiImpl, HttpMethod, Implementation, Message, RpcCall, Step, VariableDefinition};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimeTravelTrace {
  pub frames: Vec<TimeTravelFrame>,
}

impl TimeTravelTrace {
  pub fn seek(&self, index: usize) -> Option<&TimeTravelFrame> {
    self.frames.get(index)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimeTravelFrame {
  pub index: usize,
  pub implementation: String,
  pub endpoint: String,
  pub step_index: usize,
  pub operation: String,
  pub reads: Vec<String>,
  pub writes: Vec<String>,
}

pub fn build_time_travel_trace(implementations: &[Implementation]) -> TimeTravelTrace {
  let mut frames = Vec::new();

  for implementation in implementations {
    let Implementation::PublishHttpApi(api) = implementation else {
      continue;
    };
    append_http_api_frames(api, &mut frames);
  }

  TimeTravelTrace { frames }
}

fn append_http_api_frames(api: &HttpApiImpl, frames: &mut Vec<TimeTravelFrame>) {
  let Some(flow) = &api.flow else {
    return;
  };

  for (step_index, step) in flow.steps.iter().enumerate() {
    frames.push(TimeTravelFrame {
      index: frames.len(),
      implementation: api.name.clone(),
      endpoint: endpoint_label(api),
      step_index,
      operation: operation_label(step),
      reads: reads_for_step(step),
      writes: writes_for_step(step),
    });
  }
}

fn endpoint_label(api: &HttpApiImpl) -> String {
  format!(
    "{} {}",
    method_label(&api.endpoint.method),
    api.endpoint.path
  )
}

fn method_label(method: &HttpMethod) -> String {
  match method {
    HttpMethod::GET => "GET".to_string(),
    HttpMethod::POST => "POST".to_string(),
    HttpMethod::PUT => "PUT".to_string(),
    HttpMethod::DELETE => "DELETE".to_string(),
    HttpMethod::PATCH => "PATCH".to_string(),
    HttpMethod::HEAD => "HEAD".to_string(),
    HttpMethod::OPTIONS => "OPTIONS".to_string(),
    HttpMethod::TRACE => "TRACE".to_string(),
    HttpMethod::CUSTOM(method) => method.clone(),
  }
}

fn operation_label(step: &Step) -> String {
  match step {
    Step::MethodCall(call) => format!("{}.{}", call.object, call.method),
    Step::Message(message) => format!("{} -> {}", message.from, message.topic),
    Step::RpcCall(call) => format!("{} -> {}", call.from, call.to),
  }
}

fn reads_for_step(step: &Step) -> Vec<String> {
  match step {
    Step::MethodCall(call) => variable_labels(&call.parameters),
    Step::Message(message) => message_reads(message),
    Step::RpcCall(call) => rpc_reads(call),
  }
}

fn writes_for_step(step: &Step) -> Vec<String> {
  match step {
    Step::MethodCall(call) => call
      .return_type
      .as_ref()
      .map(variable_label)
      .into_iter()
      .collect(),
    Step::Message(_) | Step::RpcCall(_) => Vec::new(),
  }
}

fn variable_labels(variables: &[VariableDefinition]) -> Vec<String> {
  variables.iter().map(variable_label).collect()
}

fn variable_label(variable: &VariableDefinition) -> String {
  format!("{}:{}", variable.name, variable.type_type)
}

fn message_reads(message: &Message) -> Vec<String> {
  if message.message.is_empty() {
    Vec::new()
  } else {
    vec![message.message.clone()]
  }
}

fn rpc_reads(call: &RpcCall) -> Vec<String> {
  variable_labels(&call.arguments)
}

#[cfg(test)]
mod tests {
  use crate::{
    build_time_travel_trace, Flow, HttpApiImpl, HttpEndpoint, HttpMethod, Implementation,
    MethodCall, Step, VariableDefinition,
  };

  #[test]
  fn builds_replayable_frames_for_http_flow_steps() {
    let mut api = HttpApiImpl::new("BookTicket".to_string());
    api.endpoint = HttpEndpoint {
      method: HttpMethod::POST,
      path: "/tickets".to_string(),
      ..Default::default()
    };
    api.flow = Some(Flow {
      steps: vec![
        Step::MethodCall(MethodCall {
          object: "UserRepository".to_string(),
          method: "getUserById".to_string(),
          return_type: Some(VariableDefinition {
            name: "user".to_string(),
            type_type: "User".to_string(),
            initializer: None,
          }),
          ..Default::default()
        }),
        Step::MethodCall(MethodCall {
          object: "TicketRepository".to_string(),
          method: "save".to_string(),
          parameters: vec![VariableDefinition {
            name: "user".to_string(),
            type_type: "User".to_string(),
            initializer: None,
          }],
          return_type: Some(VariableDefinition {
            name: "ticket".to_string(),
            type_type: "Ticket".to_string(),
            initializer: None,
          }),
          ..Default::default()
        }),
      ],
      ..Default::default()
    });

    let trace = build_time_travel_trace(&[Implementation::PublishHttpApi(api)]);

    assert_eq!(trace.frames.len(), 2);
    assert_eq!(trace.frames[0].implementation, "BookTicket");
    assert_eq!(trace.frames[0].step_index, 0);
    assert_eq!(trace.frames[0].endpoint, "POST /tickets");
    assert_eq!(trace.frames[0].operation, "UserRepository.getUserById");
    assert_eq!(trace.frames[0].writes, vec!["user:User"]);
    assert!(trace.frames[0].reads.is_empty());

    assert_eq!(trace.frames[1].step_index, 1);
    assert_eq!(trace.frames[1].operation, "TicketRepository.save");
    assert_eq!(trace.frames[1].reads, vec!["user:User"]);
    assert_eq!(trace.frames[1].writes, vec!["ticket:Ticket"]);
  }

  #[test]
  fn can_seek_to_a_trace_frame_by_index() {
    let mut api = HttpApiImpl::new("Empty".to_string());
    api.flow = Some(Flow {
      steps: vec![Step::MethodCall(MethodCall {
        object: "Object".to_string(),
        method: "method".to_string(),
        ..Default::default()
      })],
      ..Default::default()
    });

    let trace = build_time_travel_trace(&[Implementation::PublishHttpApi(api)]);

    let frame = trace.seek(0).expect("first frame");
    assert_eq!(frame.operation, "Object.method");
    assert!(trace.seek(1).is_none());
  }
}
