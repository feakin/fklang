use indexmap::IndexMap;
use rocket::{get, post, put, delete, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use fkl_parser::mir::{ContextMap, Entity};
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
  let fields = &entity.fields;
  let map = FakeValue::fields(fields);
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
    let fields = &entity.fields;
    vec.push(FakeValue::fields(fields));
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
  let fields = &entity.fields;
  let map = FakeValue::fields(fields);
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
  let fields = &entity.fields;
  let map = FakeValue::fields(fields);
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
  let fields = &entity.fields;
  let map = FakeValue::fields(fields);
  return Ok(Json(vec![map]));
}
