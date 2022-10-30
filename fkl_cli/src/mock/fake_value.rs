use chrono::{Date, NaiveDate};
use rand::Rng;
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
pub fn mock_value(fields: Vec<Field>) {}

pub fn mock_by_type(type_type: String) {}

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
    let mut rng = rand::thread_rng();
    let year: i32 = rng.gen_range(1970..2100);
    let day: u32 = rng.gen_range(1..365);
    let hour: u32 = rng.gen_range(0..23);
    let minute: u32 = rng.gen_range(0..59);
    let second: u32 = rng.gen_range(0..59);

    let date = NaiveDate::from_yo(year, day).and_hms(hour, minute, second);
    let time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(date, chrono::Utc);
    MockType::DateTime(time)
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
    println!("{:?}", n);
    assert!(n.float() >= 1.0);
  }

  #[test]
  fn test_range_string() {
    let n = RandomValue::range_string(1, 10);
    println!("{:?}", n);
    assert!(n.string().len() >= 1);
  }

  #[test]
  fn test_boolean() {
    let n = RandomValue::boolean();
    println!("{:?}", n);
    assert!(n.boolean());
  }

  #[test]
  fn test_datetime() {
    let n = RandomValue::datetime();
    println!("{:?}", n);
    assert!(n.datetime().year() >= 1970);
  }
}
