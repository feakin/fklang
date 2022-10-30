use figment::Figment;
use figment::providers::Serialized;
use rocket::{Build, get, info, Rocket, routes, State};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use fkl_parser::mir::ContextMap;

pub use super::stub_aggregate_api;

#[get("/")]
pub(crate) async fn index(conf: &State<MockServerConfig>) -> Json<ContextMap> {
  Json(conf.context_map.clone())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MockServerConfig {
  pub port: u32,
  pub context_map: ContextMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  pub msg: String,
}


pub fn feakin_rocket(context_map: &ContextMap) -> Rocket<Build> {
  let server_config = MockServerConfig {
    port: 8080,
    context_map: context_map.clone()
  };

  let figment = Figment::from(rocket::Config::default())
    .merge(Serialized::from(server_config, "default"));

  let port: usize = figment.extract_inner("port").unwrap();
  let url = format!("http://localhost:{}", port);
  info!("Feakin mock server is running at {}", url);

  rocket::custom(figment)
    .mount("/", routes![
      index
    ])
    .mount("/api", routes![
      stub_aggregate_api::get_aggregate_by_id,
    ])
    .attach(AdHoc::config::<MockServerConfig>())
}

#[cfg(test)]
mod test {
  use rocket::http::Status;
  use rocket::local::blocking::Client;

  use fkl_parser::mir::ContextMap;

  use crate::mock::stub_server::feakin_rocket;

  #[test]
  fn hello_world() {
    let context_map = ContextMap::default();
    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
  }

  #[test]
  fn movie_api() {
    let context_map = ContextMap::default();
    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/api/movie/1").dispatch();

    assert_eq!(response.status(), Status::Ok);
  }
}
