use fkl_parser::mir::Field;

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
pub fn mock_value(fields: Vec<Field>) {

}

pub fn mock_by_type(type_type: String) {

}

// mock strategy

