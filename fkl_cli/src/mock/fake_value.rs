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
    let n: i64 = rng.gen_range(min..max);

    MockType::Integer(n)
  }
}

#[cfg(test)]
mod tests {
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
}
