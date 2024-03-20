//! Connect the client to the server using the glue mappings
//!
//! Don't forget to update public/glue.js with the actual functions.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
  /// Send a request to the Tauri Server and expect an eventual response
  #[wasm_bindgen(js_name = call_server, catch)]
  pub async fn call_server(name: String) -> Result<JsValue, JsValue>;
}

// Direct access to console.log
#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);
}
