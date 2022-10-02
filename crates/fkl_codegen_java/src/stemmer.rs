use rust_stemmers::{Algorithm, Stemmer};

pub fn past_tense(str: &String) -> String {
  let en_stemmer = Stemmer::create(Algorithm::English);
  en_stemmer.stem(str).to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_past_tense() {
    let str = "created".to_string();
    let past_tense = past_tense(&str);
    assert_eq!(past_tense, "creat");
  }
}
