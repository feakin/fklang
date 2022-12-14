use indexmap::IndexMap;
use rocket::{delete, get, post, put, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

use fkl_mir::{ContextMap, Entity};

use crate::mock::fake_value::FakeValue;
use crate::mock::mock_type::MockType;
use crate::mock::stub_server::{ApiError, MockServerConfig};

fn filter_entity(aggregate_name: &str, entity_name: &str, context_map: &ContextMap) -> Option<Entity> {
  for bc in &context_map.contexts {
    for aggregate in &bc.aggregates {
      if aggregate.name.to_lowercase() == aggregate_name.to_lowercase() {
        for entity in &aggregate.entities {
          if entity.name.to_lowercase() == entity_name.to_lowercase() {
            return Some(entity.clone());
          }
        }
      }
    }
  }

  None
}

#[allow(unused_variables)]
#[get("/<aggregate_name>/<entity_name>/<id>")]
pub async fn get_aggregate_by_id(
  aggregate_name: &str,
  entity_name: &str,
  id: usize,
  config: &State<MockServerConfig>,
) -> Result<Json<Vec<IndexMap<String, MockType>>>, NotFound<Json<ApiError>>> {
  let opt_entity = filter_entity(aggregate_name, entity_name, &config.context_map);
  if let None = opt_entity {
    return Err(NotFound(Json(ApiError {
      msg: format!("Entity {} not found", entity_name),
    })));
  }

  let entity = opt_entity.unwrap();
  let map = mock_value_from_entity(&entity, &config.context_map);
  return Ok(Json(vec![map]));
}

#[get("/<aggregate_name>/<entity_name>")]
pub async fn get_entities(
  aggregate_name: &str,
  entity_name: &str,
  config: &State<MockServerConfig>,
) -> Result<Json<Vec<IndexMap<String, MockType>>>, NotFound<Json<ApiError>>> {
  let opt_entity = filter_entity(aggregate_name, entity_name, &config.context_map);
  if let None = opt_entity {
    return Err(NotFound(Json(ApiError {
      msg: format!("Entity {} not found", entity_name),
    })));
  }

  let entity = opt_entity.unwrap();
  let mut vec = vec![];
  for _ in 0..20 {
    let map = mock_value_from_entity(&entity, &config.context_map);
    vec.push(map);
  }
  return Ok(Json(vec));
}

#[put("/<aggregate_name>/<entity_name>")]
pub async fn create_entity(
  aggregate_name: &str,
  entity_name: &str,
  config: &State<MockServerConfig>,
) -> Result<Json<Vec<IndexMap<String, MockType>>>, NotFound<Json<ApiError>>> {
  let opt_entity = filter_entity(aggregate_name, entity_name, &config.context_map);
  if let None = opt_entity {
    return Err(NotFound(Json(ApiError {
      msg: format!("Entity {} not found", entity_name),
    })));
  }

  let entity = opt_entity.unwrap();
  let map = mock_value_from_entity(&entity, &config.context_map);
  return Ok(Json(vec![map]));
}

// update entity
#[allow(unused_variables)]
#[post("/<aggregate_name>/<entity_name>/<id>")]
pub async fn update_entity(
  aggregate_name: &str,
  entity_name: &str,
  id: usize,
  config: &State<MockServerConfig>,
) -> Result<Json<Vec<IndexMap<String, MockType>>>, NotFound<Json<ApiError>>> {
  let opt_entity = filter_entity(aggregate_name, entity_name, &config.context_map);
  if let None = opt_entity {
    return Err(NotFound(Json(ApiError {
      msg: format!("Entity {} not found", entity_name),
    })));
  }

  let entity = opt_entity.unwrap();
  let map = mock_value_from_entity(&entity, &config.context_map);
  return Ok(Json(vec![map]));
}

// delete entity
#[allow(unused_variables)]
#[delete("/<aggregate_name>/<entity_name>/<id>")]
pub async fn delete_entity(
  aggregate_name: &str,
  entity_name: &str,
  id: usize,
  config: &State<MockServerConfig>,
) -> Result<Json<Vec<IndexMap<String, MockType>>>, NotFound<Json<ApiError>>> {
  let opt_entity = filter_entity(aggregate_name, entity_name, &config.context_map);
  if let None = opt_entity {
    return Err(NotFound(Json(ApiError {
      msg: format!("Entity {} not found", entity_name),
    })));
  }

  let entity = opt_entity.unwrap();
  let map = mock_value_from_entity(&entity, &config.context_map);
  return Ok(Json(vec![map]));
}

fn mock_value_from_entity(entity: &Entity, bcs: &ContextMap) -> IndexMap<String, MockType> {
  let fields = &entity.fields;
  FakeValue::fake_with_custom(fields, &bcs.structs)
}

#[cfg(test)]
mod tests {
  use crate::builtin::funcs::mir_from_str;
  use crate::mock::mock_type::MockType;
  use crate::mock::stub_aggregate_api::mock_value_from_entity;

  #[test]
  fn simple_entity() {
    let context_map = mir_from_str("
ContextMap { Account <-> Sample }

Context Account {
  Aggregate Account {
    Entity User {
      struct {
        id: String
        name: String
        age: Int
        address: Address
        is_active: Boolean
        created_at: DateTime
        updated_at: DateTime
      }
    }
  }
}

struct Address {
  street: String
  city: String
  state: String
  zip: String
}
    ");

    let entity = context_map.get_entity("User").unwrap();
    let from_entity = mock_value_from_entity(&entity, &context_map);

    assert_eq!(from_entity.len(), 7);
    let address: &MockType = from_entity.get("address").unwrap();
    assert_eq!(address.as_map().len(), 4);
  }
}
