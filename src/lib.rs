// sample code below was taken from https://github.com/rustwasm/wasm-bindgen

extern crate wasm_bindgen;
extern crate image;

use std::io::BufReader;

use wasm_bindgen::prelude::*;
use image::{EncodableLayout, io::Reader as ImageReader};

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
  // #[wasm_bindgen(js_namespace = console)]
  // fn log(s: &str);
}

#[wasm_bindgen]
pub fn res(bytes: &[u8]) -> String {
  let bytes = BufReader::new(bytes);
  let (x, y) = ImageReader::new(bytes).into_dimensions().unwrap();

  return format!("{}, {}", x, y);
}
