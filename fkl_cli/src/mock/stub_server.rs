use colored::Colorize;
use figment::Figment;
use figment::providers::Serialized;
use rocket::{Build, get, Rocket, routes, State};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use fkl_parser::default_config;
use fkl_parser::mir::{Aggregate, ContextMap, Entity};

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
  let server_config = merge_config(context_map);

  let figment = Figment::from(rocket::Config::default())
    .merge(Serialized::from(server_config, "default"));

  let port: usize = figment.extract_inner("port").unwrap();
  let url = format!("http://localhost:{}", port);
  println!("Running at {} !", url.blue());


  println!("{}: ", "Routes".red());
  gen_api_list(context_map).iter().for_each(|api| {
    println!("{}{}", &url.green(), api.green());
  });

  rocket::custom(figment)
    .mount("/", routes![
      index
    ])
    .mount("/api", routes![
      stub_aggregate_api::get_aggregate_by_id,
      stub_aggregate_api::get_entities,
      stub_aggregate_api::create_entity,
      stub_aggregate_api::update_entity,
      stub_aggregate_api::delete_entity,
    ])
    .attach(AdHoc::config::<MockServerConfig>())
}

fn merge_config(context_map: &ContextMap) -> MockServerConfig {
  let port: u32 = if context_map.envs.len() > 0 {
    context_map.envs[0].server.port
  } else {
    default_config::SERVER_PORT
  } as u32;

  let server_config = MockServerConfig {
    port,
    context_map: context_map.clone(),
  };
  server_config
}

pub fn gen_api_list(context_map: &ContextMap) -> Vec<String> {
  context_map.contexts.iter().flat_map(|env| {
    env.aggregates.iter().flat_map(|aggregate| {
      aggregate.entities.iter().flat_map(|entity| {
        collect_apis(aggregate, entity)
      })
    })
  }).collect()
}

fn collect_apis(aggregate: &Aggregate, entity: &Entity) -> Vec<String> {
  let aggregate_name = aggregate.name.to_lowercase();
  let entity_name = entity.name.to_lowercase();

  vec![
    format!("/api/{}/{}", aggregate_name, entity_name),
    format!("/api/{}/{}/1", aggregate_name, entity_name),
  ]
}

#[cfg(test)]
mod test {
  use rocket::http::Status;
  use rocket::local::blocking::Client;

  use fkl_parser::mir::ContextMap;
  use fkl_parser::parse;

  use crate::mock::stub_server::{feakin_rocket, gen_api_list};

  fn source_code() -> &'static str {
    r#"ContextMap TicketBooking {
  TicketContext <-> ReservationContext;
}

Context TicketContext {
  Aggregate Ticket, Reservation;
}

Aggregate Ticket {
  Entity Ticket, Seat;
}

Entity Ticket {
  Struct {
    id: UUID;
    seat: String;
    price: Int;
  }
}

Entity Seat {
  Struct {
    id: UUID;
    row: Int;
    number: Int;
  }
}
"#
  }

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
    let context_map: ContextMap = parse(source_code()).unwrap();
    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/api/ticket/ticket/1").dispatch();

    assert_eq!(response.status(), Status::Ok);
  }

  #[test]
  fn api_list() {
    let context_map: ContextMap = parse(source_code()).unwrap();
    let api_list = gen_api_list(&context_map);

    assert_eq!(api_list, vec![
      "/api/ticket/ticket".to_string(),
      "/api/ticket/ticket/1".to_string(),
      "/api/ticket/seat".to_string(),
      "/api/ticket/seat/1".to_string(),
    ]);
  }

  #[test]
  fn crud_for_api() {
    let context_map: ContextMap = parse(source_code()).unwrap();

    let client = Client::tracked(feakin_rocket(&context_map)).expect("valid rocket instance");
    let response = client.get("/api/ticket/ticket/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.put("/api/ticket/seat").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.post("/api/ticket/seat/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let response = client.delete("/api/ticket/seat/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
  }
}
