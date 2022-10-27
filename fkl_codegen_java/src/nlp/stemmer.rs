use rust_stemmers::{Algorithm, Stemmer};

pub fn past_tense_to_normal(str: &str) -> String {
  if str.ends_with("ed") {
    return str[0..str.len() - 1].to_string();
  }

  let en_stemmer = Stemmer::create(Algorithm::English);
  en_stemmer.stem(str).to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_past_tense() {
    let str = "created".to_string();
    let past_tense = past_tense_to_normal(&str);
    assert_eq!(past_tense, "create");
  }
}
