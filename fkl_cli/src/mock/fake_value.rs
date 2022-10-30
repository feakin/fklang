use chrono::{DateTime, NaiveDate, Utc};
use rand::Rng;
use sqlx::types::uuid;

use fkl_parser::mir::Field;

use crate::mock::mock_type::MockType;

///
/// # Faker.js Style
///
/// ```javascript
/// export function createRandomUser(): User {
///   return {
///     userId: faker.datatype.uuid(),
///     username: faker.internet.userName(),
///     email: faker.internet.email(),
///     avatar: faker.image.avatar(),
///     password: faker.internet.password(),
///     birthdate: faker.date.birthdate(),
///     registeredAt: faker.date.past(),
///   };
/// }
/// ```
pub fn mock_value(_fields: Vec<Field>) {}

pub fn mock_by_type(_type_type: String) {}

pub struct RandomValue {}

impl RandomValue {
  pub fn number() -> MockType {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();

    MockType::Integer(n as i64)
  }

  pub fn range_number(min: i64, max: i64) -> MockType {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: i64 = rng.gen_range(min..max);

    MockType::Integer(n)
  }

  pub fn float() -> MockType {
    let mut rng = rand::thread_rng();
    let n: f64 = rng.gen();

    MockType::Float(n)
  }

  pub fn range_float(min: f64, max: f64) -> MockType {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: f64 = rng.gen_range(min..max);

    MockType::Float(n)
  }

  pub fn range_string(min: i64, max: i64) -> MockType {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: i64 = rng.gen_range(min..max);
    let mut s = String::new();

    for _ in 0..n {
      s.push(rng.gen_range(97..122) as u8 as char);
    }

    MockType::String(s)
  }

  pub fn boolean() -> MockType {
    let mut rng = rand::thread_rng();
    let n: bool = rng.gen();

    MockType::Boolean(n)
  }

  pub fn datetime() -> MockType {
    MockType::DateTime(Self::gen_time())
  }

  fn gen_time() -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    let year: i32 = rng.gen_range(1970..2100);
    let day: u32 = rng.gen_range(1..365);
    let hour: u32 = rng.gen_range(0..23);
    let minute: u32 = rng.gen_range(0..59);
    let second: u32 = rng.gen_range(0..59);

    let date = NaiveDate::from_yo(year, day).and_hms(hour, minute, second);
    let time: DateTime<Utc> = DateTime::from_utc(date, Utc);
    time
  }

  pub fn date() -> MockType {
    let mut rng = rand::thread_rng();
    let year: i32 = rng.gen_range(1970..2100);
    let day: u32 = rng.gen_range(1..365);

    let date = NaiveDate::from_yo(year, day);
    let time: chrono::Date<Utc> = chrono::Date::from_utc(date, Utc);
    MockType::Date(time)
  }

  pub fn timestamp() -> MockType {
    MockType::Timestamp(Self::gen_time().timestamp())
  }

  pub fn uuid() -> MockType {
    MockType::Uuid(uuid::Uuid::new_v4().to_string())
  }
}

#[cfg(test)]
mod tests {
  use chrono::Datelike;

  use super::*;

  #[test]
  fn test_random_value() {
    let n = RandomValue::number();
    println!("{:?}", n);
  }

  #[test]
  fn test_range_number() {
    let n = RandomValue::range_number(1, 10);
    assert!(n.integer() >= 1);
  }

  #[test]
  fn test_range_float() {
    let n = RandomValue::range_float(1.0, 10.0);
    assert!(n.float() >= 1.0);
  }

  #[test]
  fn test_range_string() {
    let n = RandomValue::range_string(1, 10);
    assert!(n.string().len() >= 1);
  }

  #[test]
  fn test_boolean() {
    let n = RandomValue::boolean();
    println!("{:?}", n);
  }

  #[test]
  fn test_datetime() {
    let n = RandomValue::datetime();
    assert!(n.datetime().year() >= 1970);
  }

  #[test]
  fn test_date() {
    let n = RandomValue::date();
    assert!(n.date().year() >= 1970);
  }

  #[test]
  fn test_timestamp() {
    let n = RandomValue::timestamp();
    assert!(n.timestamp() >= 0);
  }

  #[test]
  fn test_uuid() {
    let n = RandomValue::uuid();
    let uuid_validate_regex = regex::Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    assert!(uuid_validate_regex.is_match(&*n.uuid()));
  }
}
