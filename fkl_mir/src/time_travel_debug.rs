use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::{HttpApiImpl, HttpMethod, Implementation, Message, RpcCall, SourceSpan, Step, VariableDefinition};

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
  pub source: Option<TimeTravelSourceLocation>,
  pub state_before: TimeTravelStateSnapshot,
  pub state_after: TimeTravelStateSnapshot,
  pub state_diff: TimeTravelStateDiff,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimeTravelSourceLocation {
  pub path: String,
  pub start_byte: usize,
  pub end_byte: usize,
  pub start_line: usize,
  pub start_column: usize,
  pub end_line: usize,
  pub end_column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimeTravelStateSnapshot {
  pub variables: Vec<TimeTravelVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimeTravelStateDiff {
  pub read: Vec<TimeTravelVariable>,
  pub created: Vec<TimeTravelVariable>,
  pub updated: Vec<TimeTravelVariable>,
  pub removed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TimeTravelVariable {
  pub name: String,
  pub type_name: String,
  pub value: Option<String>,
}

pub fn build_time_travel_trace(implementations: &[Implementation]) -> TimeTravelTrace {
  build_time_travel_trace_with_source(implementations, None, None)
}

pub fn build_time_travel_trace_with_source(
  implementations: &[Implementation],
  source_path: Option<&str>,
  source: Option<&str>,
) -> TimeTravelTrace {
  let mut frames = Vec::new();
  let mut state = BTreeMap::new();

  for implementation in implementations {
    let Implementation::PublishHttpApi(api) = implementation else {
      continue;
    };
    append_http_api_frames(api, &mut frames, &mut state, source_path, source);
  }

  TimeTravelTrace { frames }
}

fn append_http_api_frames(
  api: &HttpApiImpl,
  frames: &mut Vec<TimeTravelFrame>,
  state: &mut BTreeMap<String, TimeTravelVariable>,
  source_path: Option<&str>,
  source: Option<&str>,
) {
  let Some(flow) = &api.flow else {
    return;
  };

  for (step_index, step) in flow.steps.iter().enumerate() {
    let index = frames.len();
    let operation = operation_label(step);
    let state_before = snapshot(state);
    let read_variables = read_variables_for_step(step, state);
    let write_variables = write_variables_for_step(step, &operation, index);
    let mut state_diff = TimeTravelStateDiff {
      read: read_variables,
      ..Default::default()
    };

    for variable in write_variables {
      if state.contains_key(&variable.name) {
        state_diff.updated.push(variable.clone());
      } else {
        state_diff.created.push(variable.clone());
      }
      state.insert(variable.name.clone(), variable);
    }

    frames.push(TimeTravelFrame {
      index,
      implementation: api.name.clone(),
      endpoint: endpoint_label(api),
      step_index,
      operation,
      reads: reads_for_step(step),
      writes: writes_for_step(step),
      source: source_location_for_step(step, source_path, source),
      state_before,
      state_after: snapshot(state),
      state_diff,
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

fn read_variables_for_step(
  step: &Step,
  state: &BTreeMap<String, TimeTravelVariable>,
) -> Vec<TimeTravelVariable> {
  match step {
    Step::MethodCall(call) => call
      .parameters
      .iter()
      .map(|variable| {
        state
          .get(&variable.name)
          .cloned()
          .unwrap_or_else(|| variable_from_definition(variable, None))
      })
      .collect(),
    Step::RpcCall(call) => call
      .arguments
      .iter()
      .map(|variable| {
        state
          .get(&variable.name)
          .cloned()
          .unwrap_or_else(|| variable_from_definition(variable, None))
      })
      .collect(),
    Step::Message(message) => {
      if message.message.is_empty() {
        Vec::new()
      } else {
        state
          .get(&message.message)
          .cloned()
          .map(|variable| vec![variable])
          .unwrap_or_else(|| {
            vec![TimeTravelVariable {
              name: message.message.clone(),
              type_name: message.message.clone(),
              value: None,
            }]
          })
      }
    }
  }
}

fn write_variables_for_step(
  step: &Step,
  operation: &str,
  index: usize,
) -> Vec<TimeTravelVariable> {
  match step {
    Step::MethodCall(call) => call
      .return_type
      .as_ref()
      .map(|variable| {
        variable_from_definition(variable, Some(format!("{}#{}", operation, index)))
      })
      .into_iter()
      .collect(),
    Step::Message(_) | Step::RpcCall(_) => Vec::new(),
  }
}

fn variable_from_definition(
  variable: &VariableDefinition,
  value: Option<String>,
) -> TimeTravelVariable {
  TimeTravelVariable {
    name: variable.name.clone(),
    type_name: variable.type_type.clone(),
    value,
  }
}

fn snapshot(state: &BTreeMap<String, TimeTravelVariable>) -> TimeTravelStateSnapshot {
  TimeTravelStateSnapshot {
    variables: state.values().cloned().collect(),
  }
}

fn source_location_for_step(
  step: &Step,
  source_path: Option<&str>,
  source: Option<&str>,
) -> Option<TimeTravelSourceLocation> {
  let span = source_span_for_step(step)?;
  let source = source?;
  let (start_line, start_column) = line_column_for_offset(source, span.start);
  let (end_line, end_column) = line_column_for_offset(source, span.end);

  Some(TimeTravelSourceLocation {
    path: source_path.unwrap_or("").to_string(),
    start_byte: span.start,
    end_byte: span.end,
    start_line,
    start_column,
    end_line,
    end_column,
  })
}

fn source_span_for_step(step: &Step) -> Option<SourceSpan> {
  match step {
    Step::MethodCall(call) => call.source_span,
    Step::Message(message) => message.source_span,
    Step::RpcCall(call) => call.source_span,
  }
}

fn line_column_for_offset(source: &str, offset: usize) -> (usize, usize) {
  let mut line = 1;
  let mut column = 1;

  for (index, character) in source.char_indices() {
    if index >= offset {
      break;
    }
    if character == '\n' {
      line += 1;
      column = 1;
    } else {
      column += 1;
    }
  }

  (line, column)
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

  #[test]
  fn frames_capture_state_snapshots_and_diffs() {
    let mut api = HttpApiImpl::new("BookTicket".to_string());
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

    assert!(trace.frames[0].state_before.variables.is_empty());
    assert_eq!(trace.frames[0].state_after.variables[0].name, "user");
    assert_eq!(trace.frames[0].state_diff.created[0].name, "user");

    assert_eq!(trace.frames[1].state_before.variables[0].name, "user");
    assert_eq!(trace.frames[1].state_diff.read[0].name, "user");
    assert_eq!(trace.frames[1].state_diff.created[0].name, "ticket");
    assert_eq!(trace.frames[1].state_after.variables.len(), 2);
  }

  #[test]
  fn frames_include_source_locations_from_step_spans() {
    let source = r#"impl BookTicket {
  endpoint {
    POST "/tickets";
    response: Ticket;
  }

  flow {
    via UserRepository::getUserById receive user: User
  }
}"#;
    let step_text = "via UserRepository::getUserById receive user: User";
    let start = source.find(step_text).expect("step text");
    let end = start + step_text.len();
    let mut api = HttpApiImpl::new("BookTicket".to_string());
    api.flow = Some(Flow {
      steps: vec![Step::MethodCall(MethodCall {
        object: "UserRepository".to_string(),
        method: "getUserById".to_string(),
        source_span: Some(crate::SourceSpan { start, end }),
        ..Default::default()
      })],
      ..Default::default()
    });

    let trace = crate::build_time_travel_trace_with_source(
      &[Implementation::PublishHttpApi(api)],
      Some("docs/samples/booking.fkl"),
      Some(source),
    );

    let location = trace.frames[0].source.as_ref().expect("source location");
    assert_eq!(location.path, "docs/samples/booking.fkl");
    assert_eq!(location.start_line, 8);
    assert_eq!(location.start_column, 5);
    assert_eq!(location.end_line, 8);
    assert!(location.end_column > location.start_column);
  }
}
