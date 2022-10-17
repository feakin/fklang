use log::info;
use fkl_parser::mir::HttpEndpoint;

pub struct EndpointRunner {
  endpoint: HttpEndpoint,
}

impl EndpointRunner {
  pub fn new(endpoint: HttpEndpoint) -> Self {
    EndpointRunner {
      endpoint
    }
  }

  pub fn send_request(&self) -> Result<(), ()> {
    let client = reqwest::Client::new();
    // match self.endpoint.method {
    //
    // }
    Ok(())
  }
}
