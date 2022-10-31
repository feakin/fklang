use indexmap::IndexMap;
use rocket::{get, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use fkl_parser::mir::{ContextMap, Entity};

use crate::mock::fake_value::fake_struct;
use crate::mock::mock_type::MockType;
use crate::mock::stub_server::{ApiError, MockServerConfig};

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
  let map = fake_struct(&entity.fields);
  return Ok(Json(vec![map]));
}

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
    vec.push(fake_struct(&entity.fields));
  }
  return Ok(Json(vec));
}

