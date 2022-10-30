use indexmap::IndexMap;
use rocket::{get, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

use crate::mock::fake_value::mock_struct;
use crate::mock::mock_type::FakeValue;
use crate::mock::stub_server::{ApiError, MockServerConfig};

#[allow(unused_variables)]
#[get("/<aggregate_name>/<entity_name>/<id>")]
pub async fn get_aggregate_by_id(
  aggregate_name: &str,
  entity_name: &str,
  id: usize,
  config: &State<MockServerConfig>,
) -> Result<Json<Vec<IndexMap<String, FakeValue>>>, NotFound<Json<ApiError>>> {
  for bc in &config.context_map.contexts {
    for aggregate in &bc.aggregates {
      if aggregate.name.to_lowercase() == aggregate_name.to_lowercase() {
        for entity in &aggregate.entities {
          if entity.name.to_lowercase() == entity_name.to_lowercase() {
            let map = mock_struct(&entity.fields);
            return Ok(Json(vec![map]));
          }
        }
      }
    }
  }

  return Err(NotFound(ApiError {
    msg: format!("Could not find aggregate with name {}", aggregate_name)
  }.into()));
}
