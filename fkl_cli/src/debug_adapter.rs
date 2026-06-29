use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use fkl_mir::{build_time_travel_trace_with_source, TimeTravelStateDiff, TimeTravelStateSnapshot, TimeTravelTrace, TimeTravelVariable};
use fkl_parser::parse;
use serde_json::{json, Value};

const THREAD_ID: i64 = 1;
const REF_BEFORE: i64 = 1;
const REF_AFTER: i64 = 2;
const REF_DIFF: i64 = 3;

#[derive(Default)]
pub struct DebugAdapter {
  trace: Option<TimeTravelTrace>,
  current_frame: usize,
  next_seq: i64,
  initial_main: Option<PathBuf>,
}

impl DebugAdapter {
  pub fn new(initial_main: Option<PathBuf>) -> Self {
    Self {
      initial_main,
      next_seq: 1,
      ..Default::default()
    }
  }

  pub fn handle_request(&mut self, request: Value) -> Vec<Value> {
    let command = request["command"].as_str().unwrap_or("");
    match command {
      "initialize" => vec![self.response(
        &request,
        true,
        json!({
          "supportsConfigurationDoneRequest": true,
          "supportsSteppingGranularity": true,
          "supportsSingleThreadExecutionRequests": true,
        }),
      )],
      "launch" => self.handle_launch(request),
      "configurationDone" => vec![self.response(&request, true, json!({}))],
      "threads" => vec![self.response(
        &request,
        true,
        json!({
          "threads": [
            { "id": THREAD_ID, "name": "FKL Time Travel Debug" }
          ]
        }),
      )],
      "stackTrace" => vec![self.response(&request, true, self.stack_trace_body())],
      "scopes" => vec![self.response(&request, true, self.scopes_body())],
      "variables" => vec![self.response(&request, true, self.variables_body(&request))],
      "next" | "stepIn" => self.handle_next(request),
      "continue" => vec![
        self.response(&request, true, json!({ "allThreadsContinued": false })),
        self.event("terminated", json!({})),
      ],
      "disconnect" => vec![
        self.response(&request, true, json!({})),
        self.event("terminated", json!({})),
      ],
      "timeTravelTrace" => vec![self.response(
        &request,
        true,
        json!({ "trace": self.trace.clone().unwrap_or_default() }),
      )],
      "timeTravelSeek" => self.handle_seek(request),
      _ => vec![self.response_with_message(
        &request,
        false,
        format!("unsupported DAP command: {}", command),
      )],
    }
  }

  fn handle_launch(&mut self, request: Value) -> Vec<Value> {
    match self.trace_from_launch_arguments(&request["arguments"]) {
      Ok(trace) => {
        self.trace = Some(trace);
        self.current_frame = 0;
        vec![
          self.response(&request, true, json!({})),
          self.event("initialized", json!({})),
          self.stopped_event("entry"),
        ]
      }
      Err(message) => vec![self.response_with_message(&request, false, message)],
    }
  }

  fn handle_next(&mut self, request: Value) -> Vec<Value> {
    let frame_count = self.trace.as_ref().map(|trace| trace.frames.len()).unwrap_or(0);
    if self.current_frame + 1 < frame_count {
      self.current_frame += 1;
      vec![
        self.response(&request, true, json!({})),
        self.stopped_event("step"),
      ]
    } else {
      vec![
        self.response(&request, true, json!({})),
        self.event("terminated", json!({})),
      ]
    }
  }

  fn handle_seek(&mut self, request: Value) -> Vec<Value> {
    let frame = request["arguments"]["frame"].as_u64().unwrap_or(0) as usize;
    let frame_count = self.trace.as_ref().map(|trace| trace.frames.len()).unwrap_or(0);
    if frame < frame_count {
      self.current_frame = frame;
      vec![
        self.response(&request, true, json!({ "frame": frame })),
        self.stopped_event("timeTravel"),
      ]
    } else {
      vec![self.response_with_message(
        &request,
        false,
        format!("frame {} is outside the trace", frame),
      )]
    }
  }

  fn trace_from_launch_arguments(&self, arguments: &Value) -> Result<TimeTravelTrace, String> {
    if let Some(source) = arguments["source"].as_str() {
      let source_path = arguments["sourcePath"].as_str().unwrap_or("inline.fkl");
      return self.trace_from_source(source_path, source);
    }

    let main = arguments["main"]
      .as_str()
      .or_else(|| arguments["program"].as_str())
      .map(PathBuf::from)
      .or_else(|| self.initial_main.clone())
      .ok_or_else(|| "launch requires `source`, `main`, or `program`".to_string())?;
    let source = fs::read_to_string(&main)
      .map_err(|err| format!("failed to read {}: {}", main.display(), err))?;

    self.trace_from_source(&main.display().to_string(), &source)
  }

  fn trace_from_source(&self, source_path: &str, source: &str) -> Result<TimeTravelTrace, String> {
    let mir = parse(source).map_err(|err| err.to_string())?;
    Ok(build_time_travel_trace_with_source(
      &mir.implementations,
      Some(source_path),
      Some(source),
    ))
  }

  fn stack_trace_body(&self) -> Value {
    let Some(frame) = self.current_trace_frame() else {
      return json!({ "stackFrames": [], "totalFrames": 0 });
    };

    let source = frame.source.as_ref();
    json!({
      "stackFrames": [
        {
          "id": frame.index as i64 + 1,
          "name": frame.operation,
          "line": source.map(|location| location.start_line).unwrap_or(1),
          "column": source.map(|location| location.start_column).unwrap_or(1),
          "source": {
            "name": source
              .and_then(|location| PathBuf::from(&location.path).file_name().map(|name| name.to_string_lossy().to_string()))
              .unwrap_or_else(|| "unknown.fkl".to_string()),
            "path": source.map(|location| location.path.clone()).unwrap_or_default()
          }
        }
      ],
      "totalFrames": 1
    })
  }

  fn scopes_body(&self) -> Value {
    let frame_id = self.current_frame as i64 + 1;
    json!({
      "scopes": [
        { "name": "State Before", "variablesReference": variable_reference(frame_id, REF_BEFORE), "expensive": false },
        { "name": "State After", "variablesReference": variable_reference(frame_id, REF_AFTER), "expensive": false },
        { "name": "State Diff", "variablesReference": variable_reference(frame_id, REF_DIFF), "expensive": false }
      ]
    })
  }

  fn variables_body(&self, request: &Value) -> Value {
    let reference = request["arguments"]["variablesReference"].as_i64().unwrap_or(0);
    let (frame_index, kind) = decode_variable_reference(reference);
    let Some(trace) = &self.trace else {
      return json!({ "variables": [] });
    };
    let Some(frame) = trace.frames.get(frame_index) else {
      return json!({ "variables": [] });
    };

    let variables = match kind {
      REF_BEFORE => variables_from_snapshot(&frame.state_before),
      REF_AFTER => variables_from_snapshot(&frame.state_after),
      REF_DIFF => variables_from_diff(&frame.state_diff),
      _ => Vec::new(),
    };

    json!({ "variables": variables })
  }

  fn current_trace_frame(&self) -> Option<&fkl_mir::TimeTravelFrame> {
    self.trace
      .as_ref()
      .and_then(|trace| trace.frames.get(self.current_frame))
  }

  fn response(&mut self, request: &Value, success: bool, body: Value) -> Value {
    let command = request["command"].as_str().unwrap_or("").to_string();
    json!({
      "seq": self.outgoing_seq(),
      "type": "response",
      "request_seq": request["seq"].as_i64().unwrap_or(0),
      "success": success,
      "command": command,
      "body": body
    })
  }

  fn response_with_message(&mut self, request: &Value, success: bool, message: String) -> Value {
    let mut response = self.response(request, success, json!({}));
    response["message"] = Value::String(message);
    response
  }

  fn event(&mut self, event: &str, body: Value) -> Value {
    json!({
      "seq": self.outgoing_seq(),
      "type": "event",
      "event": event,
      "body": body
    })
  }

  fn stopped_event(&mut self, reason: &str) -> Value {
    self.event(
      "stopped",
      json!({
        "reason": reason,
        "threadId": THREAD_ID,
        "allThreadsStopped": true
      }),
    )
  }

  fn outgoing_seq(&mut self) -> i64 {
    let seq = self.next_seq;
    self.next_seq += 1;
    seq
  }
}

pub fn run_stdio(initial_main: Option<PathBuf>) -> io::Result<()> {
  let stdin = io::stdin();
  let stdout = io::stdout();
  run(BufReader::new(stdin.lock()), stdout.lock(), initial_main)
}

fn run<R: BufRead, W: Write>(
  mut reader: R,
  mut writer: W,
  initial_main: Option<PathBuf>,
) -> io::Result<()> {
  let mut adapter = DebugAdapter::new(initial_main);

  while let Some(message) = read_dap_message(&mut reader)? {
    for response in adapter.handle_request(message) {
      writer.write_all(encode_dap_message(&response).as_bytes())?;
      writer.flush()?;
    }
  }

  Ok(())
}

pub fn encode_dap_message(message: &Value) -> String {
  let body = serde_json::to_string(message).expect("failed to encode dap json");
  format!("Content-Length: {}\r\n\r\n{}", body.as_bytes().len(), body)
}

#[cfg(test)]
pub fn decode_dap_messages(buffer: &[u8]) -> io::Result<Vec<Value>> {
  let mut reader = BufReader::new(buffer);
  let mut messages = Vec::new();

  while let Some(message) = read_dap_message(&mut reader)? {
    messages.push(message);
  }

  Ok(messages)
}

fn read_dap_message<R: BufRead>(reader: &mut R) -> io::Result<Option<Value>> {
  let mut content_length = None;
  let mut saw_header = false;

  loop {
    let mut line = String::new();
    let read = reader.read_line(&mut line)?;
    if read == 0 {
      return Ok(None);
    }
    let trimmed = line.trim_end_matches(['\r', '\n']);
    if trimmed.is_empty() {
      break;
    }
    saw_header = true;
    if let Some(length) = trimmed.strip_prefix("Content-Length:") {
      content_length = Some(length.trim().parse::<usize>().map_err(|err| {
        io::Error::new(io::ErrorKind::InvalidData, format!("invalid content length: {}", err))
      })?);
    }
  }

  if !saw_header {
    return Ok(None);
  }

  let length = content_length.ok_or_else(|| {
    io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length header")
  })?;
  let mut body = vec![0; length];
  reader.read_exact(&mut body)?;
  let value = serde_json::from_slice(&body)
    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

  Ok(Some(value))
}

fn variable_reference(frame_id: i64, kind: i64) -> i64 {
  frame_id * 10 + kind
}

fn decode_variable_reference(reference: i64) -> (usize, i64) {
  let frame_id = reference / 10;
  let kind = reference % 10;
  (frame_id.saturating_sub(1) as usize, kind)
}

fn variables_from_snapshot(snapshot: &TimeTravelStateSnapshot) -> Vec<Value> {
  snapshot
    .variables
    .iter()
    .map(|variable| dap_variable(&variable.name, variable))
    .collect()
}

fn variables_from_diff(diff: &TimeTravelStateDiff) -> Vec<Value> {
  let mut variables = Vec::new();
  variables.extend(
    diff
      .read
      .iter()
      .map(|variable| dap_variable(&format!("read.{}", variable.name), variable)),
  );
  variables.extend(
    diff
      .created
      .iter()
      .map(|variable| dap_variable(&format!("created.{}", variable.name), variable)),
  );
  variables.extend(
    diff
      .updated
      .iter()
      .map(|variable| dap_variable(&format!("updated.{}", variable.name), variable)),
  );

  variables
}

fn dap_variable(name: &str, variable: &TimeTravelVariable) -> Value {
  json!({
    "name": name,
    "value": variable.value.clone().unwrap_or_else(|| "<unresolved>".to_string()),
    "type": variable.type_name,
    "variablesReference": 0
  })
}

#[cfg(test)]
mod tests {
  use serde_json::json;

  use crate::debug_adapter::{
    decode_dap_messages, encode_dap_message, DebugAdapter,
  };

  fn sample_source() -> &'static str {
    r#"impl BookTicket {
  endpoint {
    POST "/tickets";
    response: Ticket;
  }

  flow {
    via UserRepository::getUserById receive user: User
    via TicketRepository::save(user: User) receive ticket: Ticket;
  }
}"#
  }

  #[test]
  fn dap_framing_round_trips_content_length_messages() {
    let message = json!({
      "seq": 1,
      "type": "request",
      "command": "initialize"
    });

    let encoded = encode_dap_message(&message);
    let decoded = decode_dap_messages(encoded.as_bytes()).expect("decoded messages");

    assert_eq!(decoded, vec![message]);
  }

  #[test]
  fn handles_initialize_launch_and_stack_trace_requests() {
    let mut adapter = DebugAdapter::default();

    let initialize = adapter.handle_request(json!({
      "seq": 1,
      "type": "request",
      "command": "initialize"
    }));
    assert_eq!(initialize[0]["success"], true);
    assert_eq!(
      initialize[0]["body"]["supportsConfigurationDoneRequest"],
      true
    );

    let launch = adapter.handle_request(json!({
      "seq": 2,
      "type": "request",
      "command": "launch",
      "arguments": {
        "source": sample_source(),
        "sourcePath": "booking.fkl"
      }
    }));
    assert_eq!(launch[0]["success"], true);
    assert_eq!(launch[1]["event"], "initialized");
    assert_eq!(launch[2]["event"], "stopped");

    let stack = adapter.handle_request(json!({
      "seq": 3,
      "type": "request",
      "command": "stackTrace",
      "arguments": { "threadId": 1 }
    }));
    let frame = &stack[0]["body"]["stackFrames"][0];
    assert_eq!(frame["name"], "UserRepository.getUserById");
    assert_eq!(frame["source"]["path"], "booking.fkl");
    assert_eq!(frame["line"], 8);
  }

  #[test]
  fn steps_and_seeks_between_time_travel_frames() {
    let mut adapter = DebugAdapter::default();
    adapter.handle_request(json!({
      "seq": 1,
      "type": "request",
      "command": "launch",
      "arguments": {
        "source": sample_source(),
        "sourcePath": "booking.fkl"
      }
    }));

    let next = adapter.handle_request(json!({
      "seq": 2,
      "type": "request",
      "command": "next",
      "arguments": { "threadId": 1 }
    }));
    assert_eq!(next[0]["success"], true);
    assert_eq!(next[1]["body"]["reason"], "step");

    let stack = adapter.handle_request(json!({
      "seq": 3,
      "type": "request",
      "command": "stackTrace",
      "arguments": { "threadId": 1 }
    }));
    assert_eq!(stack[0]["body"]["stackFrames"][0]["name"], "TicketRepository.save");

    let seek = adapter.handle_request(json!({
      "seq": 4,
      "type": "request",
      "command": "timeTravelSeek",
      "arguments": { "frame": 0 }
    }));
    assert_eq!(seek[0]["success"], true);

    let trace = adapter.handle_request(json!({
      "seq": 5,
      "type": "request",
      "command": "timeTravelTrace"
    }));
    assert_eq!(trace[0]["body"]["trace"]["frames"].as_array().unwrap().len(), 2);
  }
}
