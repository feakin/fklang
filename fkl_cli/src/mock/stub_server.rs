use figment::Figment;
use figment::providers::Serialized;
use rocket::{Build, get, info, Rocket, routes, State};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use fkl_parser::default_config;

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
  let port: u32 = if context_map.envs.len() > 0 {
    context_map.envs[0].server.port
  } else {
    default_config::SERVER_PORT
  } as u32;

  let server_config = MockServerConfig {
    port,
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
      stub_aggregate_api::get_entities,
    ])
    .attach(AdHoc::config::<MockServerConfig>())
}

#[cfg(test)]
mod test {
  use rocket::http::Status;
  use rocket::local::blocking::Client;

  use fkl_parser::mir::ContextMap;
  use fkl_parser::parse;

  use crate::mock::stub_server::feakin_rocket;

  #[test]
  fn sample() {
    let context_map = ContextMap::default();
    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
  }

  #[test]
  fn return_404_for_no_exist_struct() {
    let context_map = ContextMap::default();
    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/api/movie/movie/1").dispatch();

    assert_eq!(response.status(), Status::NotFound);
  }

  #[test]
  fn return_ok_for_exist_aggregate_struct() {
    let source = r#"ContextMap TicketBooking {
  TicketContext <-> ReservationContext;
}

Context TicketContext {
  Aggregate Ticket, Reservation;
}

Aggregate Ticket {
  Entity Ticket;
}

Entity Ticket {
  Struct {
    id: UUID;
    seat: String;
    price: Int;
  }
}
"#;

    let context_map: ContextMap = parse(source).unwrap();
    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/api/ticket/ticket/1").dispatch();

    assert_eq!(response.status(), Status::Ok);
  }
}
