use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
  CompletionItem, CompletionItemKind, CompletionOptions, CompletionResponse, Diagnostic,
  DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
  DidOpenTextDocumentParams, Documentation, Hover, HoverContents, HoverProviderCapability,
  InitializeParams, InitializeResult, InitializedParams, InsertTextFormat, MarkedString,
  MessageType, Position, Range, ServerCapabilities, TextDocumentSyncCapability,
  TextDocumentSyncKind, Url,
};
use tower_lsp::{Client, LanguageServer};

const FKL_KEYWORDS: &[(&str, &str)] = &[
  (
    "ContextMap",
    "Defines a map of bounded contexts and their relationships.",
  ),
  ("Context", "Defines a bounded context."),
  ("Aggregate", "Defines an aggregate boundary."),
  ("Entity", "Defines an entity inside a domain model."),
  ("ValueObject", "Defines an immutable value object."),
  ("VO", "Short form for ValueObject."),
  (
    "Struct",
    "Defines fields for an entity, value object, or aggregate sugar.",
  ),
  ("impl", "Defines an implementation endpoint or flow."),
  ("endpoint", "Defines an HTTP endpoint implementation."),
  ("flow", "Defines implementation flow steps."),
  ("layered", "Defines a layered architecture rule set."),
  (
    "layer",
    "Defines one layer inside layered architecture rules.",
  ),
  ("dependency", "Defines allowed layer dependencies."),
  ("SourceSet", "Defines external source sets."),
  ("env", "Defines environment resources."),
];

#[derive(Debug)]
pub struct Backend {
  client: Client,
  documents: DashMap<Url, String>,
}

impl Backend {
  pub fn new(client: Client) -> Self {
    Self {
      client,
      documents: DashMap::new(),
    }
  }

  async fn publish_diagnostics(&self, uri: Url, text: &str) {
    self
      .client
      .publish_diagnostics(uri, diagnostics_for_text(text), None)
      .await;
  }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
  async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
    Ok(InitializeResult {
      capabilities: server_capabilities(),
      ..InitializeResult::default()
    })
  }

  async fn initialized(&self, _: InitializedParams) {
    self
      .client
      .log_message(MessageType::INFO, "FKL language server initialized")
      .await;
  }

  async fn did_open(&self, params: DidOpenTextDocumentParams) {
    let uri = params.text_document.uri;
    let text = params.text_document.text;
    self.documents.insert(uri.clone(), text.clone());
    self.publish_diagnostics(uri, &text).await;
  }

  async fn did_change(&self, params: DidChangeTextDocumentParams) {
    let uri = params.text_document.uri;
    if let Some(change) = params.content_changes.into_iter().last() {
      self.documents.insert(uri.clone(), change.text.clone());
      self.publish_diagnostics(uri, &change.text).await;
    }
  }

  async fn did_close(&self, params: DidCloseTextDocumentParams) {
    let uri = params.text_document.uri;
    self.documents.remove(&uri);
    self.client.publish_diagnostics(uri, Vec::new(), None).await;
  }

  async fn completion(
    &self,
    _: tower_lsp::lsp_types::CompletionParams,
  ) -> Result<Option<CompletionResponse>> {
    Ok(Some(CompletionResponse::Array(completion_items())))
  }

  async fn hover(&self, params: tower_lsp::lsp_types::HoverParams) -> Result<Option<Hover>> {
    let text_document = params.text_document_position_params.text_document;
    let position = params.text_document_position_params.position;

    Ok(
      self
        .documents
        .get(&text_document.uri)
        .and_then(|text| hover_for_position(&text, position)),
    )
  }

  async fn shutdown(&self) -> Result<()> {
    Ok(())
  }
}

pub fn server_capabilities() -> ServerCapabilities {
  ServerCapabilities {
    text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
    completion_provider: Some(CompletionOptions::default()),
    hover_provider: Some(HoverProviderCapability::Simple(true)),
    ..ServerCapabilities::default()
  }
}

pub fn diagnostics_for_text(text: &str) -> Vec<Diagnostic> {
  match fkl_parser::ast_parse(text) {
    Ok(_) => Vec::new(),
    Err(error) => vec![Diagnostic {
      range: range_from_parse_error(&error.to_string(), text),
      severity: Some(DiagnosticSeverity::ERROR),
      source: Some("fkl_parser".to_string()),
      message: error.to_string(),
      ..Diagnostic::default()
    }],
  }
}

pub fn completion_items() -> Vec<CompletionItem> {
  vec![
    keyword("ContextMap", "ContextMap ${1:Name} {\n  ${0}\n}"),
    keyword("Context", "Context ${1:Name} {\n  ${0}\n}"),
    keyword("Aggregate", "Aggregate ${1:Name} {\n  ${0}\n}"),
    keyword("Entity", "Entity ${1:Name} {\n  ${0}\n}"),
    keyword("ValueObject", "ValueObject ${1:Name} {\n  ${0}\n}"),
    keyword("Struct", "Struct {\n  ${1:field}: ${2:String};\n}"),
    keyword(
      "impl",
      "impl ${1:Name} {\n  endpoint {\n    GET \"${2:/}\";\n    response: ${3:String};\n  }\n}",
    ),
    keyword(
      "endpoint",
      "endpoint {\n  GET \"${1:/}\";\n  response: ${2:String};\n}",
    ),
    keyword(
      "flow",
      "flow {\n  via ${1:Object} receive ${2:value}: ${3:Type};\n}",
    ),
    keyword("layered", "layered ${1:Name} {\n  ${0}\n}"),
    keyword(
      "layer",
      "layer ${1:Name} {\n  package: \"${2:package}\";\n}",
    ),
    keyword(
      "dependency",
      "dependency {\n  ${1:source} -> ${2:target}\n}",
    ),
    keyword("SourceSet", "SourceSet ${1:Name} {\n  ${0}\n}"),
    keyword("env", "env ${1:Name} {\n  ${0}\n}"),
  ]
}

pub fn hover_for_position(text: &str, position: Position) -> Option<Hover> {
  let word = word_at_position(text, position)?;
  let (_, description) = FKL_KEYWORDS.iter().find(|(keyword, _)| *keyword == word)?;

  Some(Hover {
    contents: HoverContents::Scalar(MarkedString::String(format!("{}: {}", word, description))),
    range: None,
  })
}

fn keyword(label: &str, insert_text: &str) -> CompletionItem {
  CompletionItem {
    label: label.to_string(),
    kind: Some(CompletionItemKind::KEYWORD),
    detail: Some("FKL keyword".to_string()),
    documentation: FKL_KEYWORDS
      .iter()
      .find(|(keyword, _)| *keyword == label)
      .map(|(_, description)| Documentation::String(description.to_string())),
    insert_text: Some(insert_text.to_string()),
    insert_text_format: Some(InsertTextFormat::SNIPPET),
    ..CompletionItem::default()
  }
}

fn range_from_parse_error(message: &str, text: &str) -> Range {
  let Some((line, character)) = parse_pest_location(message) else {
    return Range::new(Position::new(0, 0), Position::new(0, 1));
  };

  let max_character = line_len(text, line);
  let character = character.min(max_character);
  Range::new(
    Position::new(line, character),
    Position::new(line, (character + 1).min(max_character.saturating_add(1))),
  )
}

fn parse_pest_location(message: &str) -> Option<(u32, u32)> {
  for line in message.lines() {
    let location = line.trim().strip_prefix("--> ")?;
    let (line, character) = location.split_once(':')?;
    return Some((
      line.parse::<u32>().ok()?.saturating_sub(1),
      character.parse::<u32>().ok()?.saturating_sub(1),
    ));
  }
  None
}

fn line_len(text: &str, line: u32) -> u32 {
  text
    .lines()
    .nth(line as usize)
    .map(|line| line.chars().count() as u32)
    .unwrap_or(0)
}

fn word_at_position(text: &str, position: Position) -> Option<String> {
  let line = text.lines().nth(position.line as usize)?;
  let chars: Vec<char> = line.chars().collect();
  let mut index = (position.character as usize).min(chars.len());

  if index == chars.len() && index > 0 {
    index -= 1;
  }

  if !is_word_char(*chars.get(index)?) {
    return None;
  }

  let mut start = index;
  while start > 0 && is_word_char(chars[start - 1]) {
    start -= 1;
  }

  let mut end = index + 1;
  while end < chars.len() && is_word_char(chars[end]) {
    end += 1;
  }

  Some(chars[start..end].iter().collect())
}

fn is_word_char(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
  use tower_lsp::lsp_types::{
    CompletionItemKind, DiagnosticSeverity, HoverContents, MarkedString, Position,
    TextDocumentSyncCapability, TextDocumentSyncKind,
  };

  use crate::{completion_items, diagnostics_for_text, hover_for_position, server_capabilities};

  #[test]
  fn advertises_core_fkl_lsp_capabilities() {
    let capabilities = server_capabilities();

    assert_eq!(
      capabilities.text_document_sync,
      Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL))
    );
    assert!(capabilities.completion_provider.is_some());
    assert!(capabilities.hover_provider.is_some());
  }

  #[test]
  fn valid_documents_have_no_parse_diagnostics() {
    let diagnostics = diagnostics_for_text(
      r#"ContextMap TicketBooking {
  Reservation -> Cinema;
}
"#,
    );

    assert!(diagnostics.is_empty());
  }

  #[test]
  fn invalid_documents_report_a_syntax_diagnostic_at_the_parser_location() {
    let diagnostics = diagnostics_for_text("ContextMap Demo {\n  Reservation ->\n}\n");

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    assert!(diagnostics[0].message.contains("expected"));
    assert_eq!(diagnostics[0].range.start, Position::new(2, 0));
    assert_eq!(diagnostics[0].range.end, Position::new(2, 1));
  }

  #[test]
  fn completion_items_include_fkl_keywords_and_snippets() {
    let items = completion_items();

    let context_map = items
      .iter()
      .find(|item| item.label == "ContextMap")
      .expect("ContextMap completion");
    assert_eq!(context_map.kind, Some(CompletionItemKind::KEYWORD));

    let aggregate = items
      .iter()
      .find(|item| item.label == "Aggregate")
      .expect("Aggregate snippet");
    assert!(aggregate
      .insert_text
      .as_ref()
      .expect("insert text")
      .contains("Aggregate ${1:Name}"));
  }

  #[test]
  fn hover_returns_keyword_documentation_for_the_word_at_position() {
    let hover = hover_for_position("ContextMap Demo {}", Position::new(0, 1)).expect("hover");

    match hover.contents {
      HoverContents::Scalar(MarkedString::String(text)) => {
        assert!(text.contains("ContextMap"));
        assert!(text.contains("bounded contexts"));
      }
      other => panic!("unexpected hover contents: {:?}", other),
    }
  }
}
