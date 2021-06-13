// sample code below was taken from https://github.com/rustwasm/wasm-bindgen

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
  log("hi from rust");
  alert(&format!("This alert was called from Rust\n\nHello, {}!\n\n\"{}\" was called as a parameter for this function from Javascript.", name, name));
}
