use chrono::{DateTime, NaiveDate, Utc};
use indexmap::IndexMap;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::types::uuid;

use fkl_parser::mir::Field;

use crate::builtin::builtin_type::BuiltinType;
use crate::mock::mock_type::FakeValue;

pub fn mock_struct(fields: &Vec<Field>) -> IndexMap<String, BuiltinType> {
  let mut map = IndexMap::new();
  for field in fields {
    map.insert(field.name.clone(), BuiltinType::from(&field.type_type));
  }

  map
}

pub fn mock_values(map: &IndexMap<String, BuiltinType>) -> IndexMap<String, FakeValue> {
  let mut result = IndexMap::new();
  for (key, value) in map {
    result.insert(key.clone(), mock_value(&value));
  }

  result
}

fn mock_value(field: &BuiltinType) -> FakeValue {
  match field {
    BuiltinType::Any => FakeValue::Unknown("any".to_owned()),
    BuiltinType::String => RandomValue::string(),
    BuiltinType::Integer => RandomValue::integer(),
    BuiltinType::Float => RandomValue::float(),
    BuiltinType::Boolean => RandomValue::boolean(),
    BuiltinType::Date => RandomValue::date(),
    BuiltinType::DateTime => RandomValue::datetime(),
    BuiltinType::Timestamp => RandomValue::timestamp(),
    BuiltinType::Array(array) => {
      let mut vec = Vec::new();
      for item in array {
        vec.push(mock_value(item));
      }
      FakeValue::Array(vec)
    }
    BuiltinType::Map(map) => {
      let mut result: IndexMap<String, FakeValue> = IndexMap::new();
      for (key, value) in map {
        result.insert(key.clone(), mock_value(&value));
      }
      FakeValue::Map(result)
    }
  }
}

pub struct RandomValue {}

impl RandomValue {
  pub fn integer() -> FakeValue {
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
    let n: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(10)
      .map(char::from)
      .collect();

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
    let n = RandomValue::integer();
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

    let ds = mock_struct(&fields);
    assert_eq!(ds.len(), 4);
    assert_eq!(ds, IndexMap::from([
      ("id".to_string(), BuiltinType::Integer),
      ("name".to_string(), BuiltinType::String),
      ("age".to_string(), BuiltinType::Integer),
      ("created_at".to_string(), BuiltinType::DateTime),
    ]));

    let values = mock_values(&ds);
    assert_eq!(values.len(), 4);
    println!("{:?}", values);
  }
}
