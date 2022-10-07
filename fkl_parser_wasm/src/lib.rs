extern crate core;

mod utils;
mod dot_gen;
mod bc_edge_style;

use wasm_bindgen::prelude::*;
use fkl_parser::parse as fkl_parse;
use crate::utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(dead_code)]
type WasmResult<T = JsValue> = Result<T, serde_wasm_bindgen::Error>;

#[wasm_bindgen]
pub fn parse(str: String) -> String {
  set_panic_hook();
  FklParser::new(str).to_dot()
}

#[wasm_bindgen]
pub struct FklParser {
  str: String,
}

#[wasm_bindgen]
impl FklParser {
  #[wasm_bindgen(constructor)]
  pub fn new(str: String) -> Self {
    Self { str }
  }

  #[wasm_bindgen]
  pub fn to_dot(&self) -> String {
    set_panic_hook();

    let ast = fkl_parse(&self.str).unwrap();
    let dot = dot_gen::to_dot(&ast);
    dot
  }

  #[wasm_bindgen]
  pub fn parse(&self) -> Result<JsValue, JsValue> {
    set_panic_hook();

    match fkl_parse(self.str.as_str()) {
      Ok(decls) => {
        let js_value = serde_wasm_bindgen::to_value(&decls)?;
        Ok(js_value)
      }
      Err(error) => {
        let error_msg = error.to_string();
        Err(serde_wasm_bindgen::to_value(&error_msg)?)
      }
    }
  }
}
