use rand::Rng;
use crate::mock::fake_value::RandomValue;

pub struct FakeCompositeType {}

#[allow(dead_code)]
const SPECIAL_CHARS: [char; 20] = ['.', '!', '#', '$', '%', '&', '\'', '*', '+', '-', '/', '=', '?', '^', '_', '`', '{', '|', '}', '~'];
const EMAIL_PROVIDERS: [&str; 10] = ["gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "aol.com", "icloud.com", "mail.com", "msn.com", "live.com", "ymail.com"];

impl FakeCompositeType {
  pub fn email() -> String {
    let mut rng = rand::thread_rng();
    let n: i64 = rng.gen_range(1..20);
    let mut email = String::new();

    let mock_type = RandomValue::range_string(1, n);
    email.push_str(mock_type.string().as_str());

    email.push('@');
    email.push_str(EMAIL_PROVIDERS[rng.gen_range(0..EMAIL_PROVIDERS.len())]);
    email
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_email() {
    let email = FakeCompositeType::email();

    assert_eq!(email.contains("@"), true);
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    assert_eq!(email_regex.is_match(email.as_str()), true);
  }
}