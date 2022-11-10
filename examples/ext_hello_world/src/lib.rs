use async_trait::async_trait;

use fkl_ext_api::custom_runner::CustomRunner;
use fkl_mir::{ContextMap, CustomEnv};

/// 1. Create ffi function, should be naming with `_fkl_create_runner`, and return a `Box<dyn CustomRunner>`
/// 2. Create a struct that implements the `CustomRunner` trait.
/// 3. In the `name` method, return the name of the runner.
/// 4. Implement the `run` method.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _fkl_create_runner() -> *mut dyn CustomRunner {
  let object = HelloWorldRunner {};
  let boxed = Box::new(object);
  Box::into_raw(boxed)
}

pub struct HelloWorldRunner {}

#[async_trait]
impl CustomRunner for HelloWorldRunner {
  fn name(&self) -> &str {
    "HelloWorldRunner"
  }

  async fn execute(&self, _context: &ContextMap, _env: &CustomEnv) {
    println!("KafkaRunner");
  }
}
