// sample code below was taken from https://github.com/rustwasm/wasm-bindgen

extern crate wasm_bindgen;
extern crate image;

use wasm_bindgen::prelude::*;
use image::ImageDecoder;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  // #[wasm_bindgen(js_namespace = console)]
  // fn log(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn res() -> String {
  return "800x800".to_string();
}
