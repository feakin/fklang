use rocket::{get, State};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::mock_server::{ApiError, MockServerConfig};

#[get("/<entry_type>/<id>")]
pub async fn get_aggregate_by_id(
  entry_type: &str,
  id: usize,
  config: &State<MockServerConfig>, ) -> Result<Json<String>, NotFound<Json<ApiError>>> {
  return Ok(Json("".to_string()));
}
