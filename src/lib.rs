extern crate wasm_bindgen;
extern crate image;

use std::io::Cursor;

// use std::io::{BufReader, Read};
use wasm_bindgen::prelude::*;
use image::{io::Reader as ImageReader, DynamicImage};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);
}

#[wasm_bindgen]
pub fn resolution(bytes: &[u8]) -> String {
  let image = ImageReader::new(Cursor::new(bytes));

  let (x, y) = image.into_dimensions().unwrap();

  return format!("{}, {}", x, y);
}

#[wasm_bindgen]
pub fn hello_world() -> String {
    alert("hi from rust");
    return "got it".to_string();
}
