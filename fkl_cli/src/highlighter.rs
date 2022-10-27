//! Prints highlighted HTML for a file to stdout.
//! Basically just wraps a body around `highlighted_html_for_file`
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn json(s: &str) {
  let ps = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();

  let syntax = ps.find_syntax_by_extension("json").unwrap();
  let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
  for line in LinesWithEndings::from(s) {
    let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
    print!("{}", escaped);
  }
}
