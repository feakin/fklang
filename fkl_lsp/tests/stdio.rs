use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn stdio_server_responds_to_initialize() {
  let mut server = Command::new(env!("CARGO_BIN_EXE_fkl_lsp"))
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("spawn fkl_lsp");

  let mut stdin = server.stdin.take().expect("stdin");
  let stdout = server.stdout.take().expect("stdout");
  let (sender, receiver) = mpsc::channel();
  thread::spawn(move || {
    let mut stdout = BufReader::new(stdout);
    while let Ok(message) = read_lsp_message(&mut stdout) {
      if sender.send(message).is_err() {
        break;
      }
    }
  });

  write_lsp_message(
    &mut stdin,
    r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":null,"capabilities":{}}}"#,
  );

  let response = receiver
    .recv_timeout(Duration::from_secs(5))
    .expect("initialize response");
  assert!(response.contains(r#""id":1"#), "{response}");
  assert!(response.contains(r#""capabilities""#), "{response}");
  assert!(response.contains(r#""completionProvider""#), "{response}");
  assert!(response.contains(r#""hoverProvider":true"#), "{response}");

  server.kill().expect("stop fkl_lsp");
  let _ = server.wait().expect("wait for fkl_lsp");
}

fn write_lsp_message(writer: &mut impl Write, body: &str) {
  write!(writer, "Content-Length: {}\r\n\r\n{}", body.len(), body).expect("write lsp message");
  writer.flush().expect("flush lsp message");
}

fn read_lsp_message(reader: &mut impl BufRead) -> std::io::Result<String> {
  let mut content_length = None;
  loop {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
      return Err(std::io::ErrorKind::UnexpectedEof.into());
    }
    let line = line.trim_end_matches(['\r', '\n']);
    if line.is_empty() {
      break;
    }
    if let Some(length) = line.strip_prefix("Content-Length: ") {
      content_length = Some(
        length
          .parse::<usize>()
          .map_err(|_| std::io::ErrorKind::InvalidData)?,
      );
    }
  }

  let length = content_length.ok_or(std::io::ErrorKind::InvalidData)?;
  let mut body = vec![0; length];
  reader.read_exact(&mut body)?;
  String::from_utf8(body).map_err(|_| std::io::ErrorKind::InvalidData.into())
}
