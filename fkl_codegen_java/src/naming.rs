use crate::nlp::past_tense_to_normal;

pub fn from_event(str: &String) -> String {
  if is_start_uppercase(str) {
    rename(str)
  } else {
    str.to_owned()
  }
}

fn is_start_uppercase(str: &String) -> bool {
  str.chars().next().unwrap().is_uppercase()
}

fn rename(str: &String) -> String {
  let words = split_words_by_uppercase(str);
  let mut words = words;
  let last_word = words.pop().unwrap();
  let string = past_tense_to_normal(&last_word.to_lowercase());
  words.insert(0, string);
  // join words
  words.join("")
}

fn split_words_by_uppercase(str: &String) -> Vec<String> {
  let mut words: Vec<String> = vec![];
  let mut word = "".to_string();
  for c in str.chars() {
    if c.is_uppercase() {
      words.push(word);
      word = "".to_string();
    }
    word.push(c);
  };

  words.push(word);

  words
}

#[cfg(test)]
mod tests {
  use crate::naming::from_event;

  #[test]
  fn test_from_event() {
    assert_eq!(from_event(&"UserCreated".to_string()), "createUser");
    assert_eq!(from_event(&"UserUpdated".to_string()), "updateUser");
    assert_eq!(from_event(&"UserDeleted".to_string()), "deleteUser");
    assert_eq!(from_event(&"User".to_string()), "user");
  }
}
