use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use rand::Rng;
use sqlx::types::uuid;

use fkl_parser::mir::Field;

use crate::builtin::builtin_type::BuiltinType;
use crate::mock::mock_type::FakeValue;

pub fn struct_to_builtin_type(fields: &Vec<Field>) -> HashMap<String, BuiltinType> {
  let mut map = HashMap::new();
  for field in fields {
    map.insert(field.name.clone(), BuiltinType::from(&field.type_type));
  }

  map
}

pub fn mock_values(fields: &Vec<Field>) -> Vec<FakeValue> {
  fields.iter()
    .map(|field| mock_value(field))
    .collect()
}

fn mock_value(field: &Field) -> FakeValue {
  match field.type_type.as_str() {
    "int" => RandomValue::number(),
    "float" => RandomValue::float(),
    "string" => RandomValue::string(),
    "boolean" => RandomValue::boolean(),
    "date" => RandomValue::date(),
    "datetime" => RandomValue::datetime(),
    "timestamp" => RandomValue::timestamp(),
    "uuid" => RandomValue::uuid(),
    &_ => FakeValue::Unknown("".to_string()),
  }
}

pub struct RandomValue {}

impl RandomValue {
  pub fn number() -> FakeValue {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();

    FakeValue::Integer(n as i64)
  }

  pub fn range_number(min: i64, max: i64) -> FakeValue {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: i64 = rng.gen_range(min..max);

    FakeValue::Integer(n)
  }

  pub fn float() -> FakeValue {
    let mut rng = rand::thread_rng();
    let n: f64 = rng.gen();

    FakeValue::Float(n)
  }

  pub fn range_float(min: f64, max: f64) -> FakeValue {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: f64 = rng.gen_range(min..max);

    FakeValue::Float(n)
  }

  pub fn string() -> FakeValue {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen();

    FakeValue::String(n.to_string())
  }

  pub fn range_string(min: i64, max: i64) -> FakeValue {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: i64 = rng.gen_range(min..max);
    let mut s = String::new();

    for _ in 0..n {
      s.push(rng.gen_range(97..122) as u8 as char);
    }

    FakeValue::String(s)
  }

  pub fn boolean() -> FakeValue {
    let mut rng = rand::thread_rng();
    let n: bool = rng.gen();

    FakeValue::Boolean(n)
  }

  pub fn datetime() -> FakeValue {
    FakeValue::DateTime(Self::gen_time())
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

  pub fn date() -> FakeValue {
    let mut rng = rand::thread_rng();
    let year: i32 = rng.gen_range(1970..2100);
    let day: u32 = rng.gen_range(1..365);

    let date = NaiveDate::from_yo(year, day);
    let time: chrono::Date<Utc> = chrono::Date::from_utc(date, Utc);
    FakeValue::Date(time)
  }

  pub fn timestamp() -> FakeValue {
    FakeValue::Timestamp(Self::gen_time().timestamp())
  }

  pub fn uuid() -> FakeValue {
    FakeValue::Uuid(uuid::Uuid::new_v4().to_string())
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

  fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
  }

  #[test]
  fn test_mock_value() {
    let fields = vec![
      Field {
        name: "id".to_string(),
        initializer: None,
        type_type: "int".to_string(),
      },
      Field {
        name: "name".to_string(),
        initializer: None,
        type_type: "string".to_string(),
      },
      Field {
        name: "age".to_string(),
        initializer: None,
        type_type: "int".to_string(),
      },
      Field {
        name: "created_at".to_string(),
        initializer: None,
        type_type: "datetime".to_string(),
      },
    ];

    let mock_values = mock_values(&fields);
    assert_eq!(mock_values.len(), 4);
    assert_eq!(type_of(&mock_values[0]), "fkl::mock::mock_type::FakeValue");
    assert_eq!(type_of(&mock_values[1]), "fkl::mock::mock_type::FakeValue");
    assert_eq!(type_of(&mock_values[2]), "fkl::mock::mock_type::FakeValue");
    assert_eq!(type_of(&mock_values[3]), "fkl::mock::mock_type::FakeValue");
  }
}
