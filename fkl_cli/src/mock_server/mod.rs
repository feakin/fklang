use rocket::{Build, info, Rocket, routes, State};
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use rocket::figment::providers::Serialized;
use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use fkl_parser::mir::ContextMap;
use rocket::serde::json::Json;

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
    .attach(AdHoc::config::<MockServerConfig>())
}
