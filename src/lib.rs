extern crate image;
extern crate imageproc;
extern crate wasm_bindgen;
extern crate web_sys;

use image::{DynamicImage, io::Reader as ImageReader};
use std::io::{Bytes, Cursor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn run(value: &[u8]) {
    let message = get_dimensions(value);

    log(&message);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let resolution = body
        .children()
        .named_item("resolution")
        .expect("couldn't find resolution");

    resolution.set_text_content(Some(&message));
}

pub fn get_dimensions(value: &[u8]) -> String {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let (x, y) = image.into_dimensions().unwrap();
    return format!("resolution: ({}, {})", x, y);
}

pub fn invert(value: &[u8]) -> &[u8] {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");
    
    let mut image = match image.decode() {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    image.invert();

    return image.as_bytes();
}