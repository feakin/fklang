use crate::highlighter::Highlighter;

pub struct CliCtx {
  pub(crate) highlighter: Highlighter,
}

impl CliCtx {
  pub fn new() -> Self {
    CliCtx {
      highlighter: Highlighter::new(),
    }
  }
}
