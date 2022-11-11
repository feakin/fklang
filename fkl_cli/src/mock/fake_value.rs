use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, Utc};
use indexmap::IndexMap;
use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::types::uuid;

use fkl_mir::{Field, Struct};

use crate::builtin::types::BuiltinType;
use crate::mock::mock_type::MockType;

pub struct FakeValue {}

impl FakeValue {
  pub fn fake_values(map: &IndexMap<String, BuiltinType>) -> IndexMap<String, MockType> {
    let mut result = IndexMap::new();
    for (key, value) in map {
      result.insert(key.clone(), FakeValue::convert_type(&value));
    }

    result
  }

  pub fn builtin_type(fields: &Vec<Field>) -> IndexMap<String, BuiltinType> {
    let mut map = IndexMap::new();
    for field in fields {
      map.insert(field.name.clone(), BuiltinType::from(&field.type_type));
    }

    map
  }

  pub fn fake(fields: &Vec<Field>) -> IndexMap<String, MockType> {
    FakeValue::fake_values(&FakeValue::builtin_type(fields))
  }

  pub fn fake_with_custom(fields: &Vec<Field>, struct_map: &HashMap<String, Struct>) -> IndexMap<String, MockType> {
    let to_types = FakeValue::builtin_type(fields);
    let mut result = IndexMap::new();
    for (key, value) in to_types {
      match &value {
        BuiltinType::Special(special) => {
          if let Some(got_struct) = struct_map.get(special) {
            let struct_fields = got_struct.fields.clone();
            let fake_value = FakeValue::fake_with_custom(&struct_fields, struct_map);
            result.insert(key.clone(), MockType::Map(fake_value));
          } else {
            result.insert(key, FakeValue::convert_type(&value));
          }
        }
        _ => {
          result.insert(key, FakeValue::convert_type(&value));
        }
      }
    }

    result
  }

  fn convert_type(field: &BuiltinType) -> MockType {
    match field {
      BuiltinType::Any => MockType::Unknown("any".to_owned()),
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
          vec.push(FakeValue::convert_type(item));
        }
        MockType::Array(vec)
      }
      BuiltinType::Map(map) => {
        let mut result: IndexMap<String, MockType> = IndexMap::new();
        for (key, value) in map {
          result.insert(key.clone(), FakeValue::convert_type(&value));
        }
        MockType::Map(result)
      }
      BuiltinType::Special(sp) => {
        match sp.as_str() {
          "uuid" => RandomValue::uuid(),
          _ => MockType::Unknown("any".to_owned())
        }
      }
    }
  }
}

pub struct RandomValue {}

impl RandomValue {
  pub fn integer() -> MockType {
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

  pub fn string() -> MockType {
    let n: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(10)
      .map(char::from)
      .collect();

    MockType::String(n.to_string())
  }

  pub fn range_string(min: i64, max: i64) -> MockType {
    let mut rng = rand::thread_rng();

    if min > max {
      panic!("min must be less than max")
    }

    let n: i64 = rng.gen_range(min..max);

    let s: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(n as usize)
      .map(char::from)
      .collect();

    MockType::String(s)
  }

  pub fn boolean() -> MockType {
    let mut rng = rand::thread_rng();
    let n: bool = rng.gen();

    MockType::Boolean(n)
  }

  pub fn datetime() -> MockType {
    MockType::DateTime(Self::gen_time().to_string())
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
    MockType::Date(time.to_string())
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
    let n = RandomValue::integer();
    println!("{:?}", n);
  }

  #[test]
  fn test_range_number() {
    let n = RandomValue::range_number(1, 10);
    assert!(n.as_integer() >= 1);
  }

  #[test]
  fn test_range_float() {
    let n = RandomValue::range_float(1.0, 10.0);
    assert!(n.as_float() >= 1.0);
  }

  #[test]
  fn test_range_string() {
    let n = RandomValue::range_string(1, 10);
    assert!(n.as_string().len() >= 1);
  }

  #[test]
  fn test_boolean() {
    let n = RandomValue::boolean();
    println!("{:?}", n);
  }

  #[test]
  fn test_datetime() {
    let n = RandomValue::datetime().as_datetime();
    let datetime_format = "%Y-%m-%d %H:%M:%S UTC";
    let dt = NaiveDate::parse_from_str(&*n, datetime_format).unwrap();
    assert!(dt.year() >= 1970);
  }

  #[test]
  fn test_date() {
    let n = RandomValue::date().as_date();
    let date_format = "%Y-%m-%dUTC";
    let dt = NaiveDate::parse_from_str(&*n, date_format).unwrap();
    assert!(dt.year() >= 1970);
  }

  #[test]
  fn test_timestamp() {
    let n = RandomValue::timestamp();
    assert!(n.as_timestamp() >= 0);
  }

  #[test]
  fn test_uuid() {
    let n = RandomValue::uuid();
    let uuid_validate_regex = regex::Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
    assert!(uuid_validate_regex.is_match(&*n.as_uuid()));
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

    let ds = FakeValue::builtin_type(&fields);
    assert_eq!(ds.len(), 4);
    assert_eq!(ds, IndexMap::from([
      ("id".to_string(), BuiltinType::Integer),
      ("name".to_string(), BuiltinType::String),
      ("age".to_string(), BuiltinType::Integer),
      ("created_at".to_string(), BuiltinType::DateTime),
    ]));

    let values = FakeValue::fake_values(&ds);
    assert_eq!(values.len(), 4);
  }
}
