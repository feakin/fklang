//! Prints highlighted HTML for a file to stdout.
//! Basically just wraps a body around `highlighted_html_for_file`
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub struct Highlighter {
  ps: SyntaxSet,
  ts: ThemeSet,
}

impl Highlighter {
  pub fn new() -> Self {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    Highlighter {
      ps: syntax_set,
      ts: theme_set,
    }
  }

  pub fn highlight_json(&self, s: &str) {
    let syntax = self.ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
      let ranges: Vec<(Style, &str)> = h.highlight_line(line, &self.ps).unwrap();
      let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
      print!("{}", escaped);
    }
  }

  pub fn json(s: &str) {
    Highlighter::new().highlight_json(s);
  }
}

