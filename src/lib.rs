extern crate image;
extern crate wasm_bindgen;
extern crate web_sys;
// extern crate js_sys;

use image::io::Reader as ImageReader;
// use js_sys::Uint8Array;
use web_sys::{HtmlElement, HtmlImageElement};
use std::io::Cursor;
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
    let image = ImageReader::new(Cursor::new(value)).with_guessed_format().expect("can't get imagereader from the image");

    let (x, y) = image.into_dimensions().expect("can't get image dimensions");
    let message = format!("({}, {})", x, y);

    log(&message);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    
    let resolution = body.children().named_item("resolution").expect("couldn't find resolution");
    let preview = body.children().named_item("preview").expect("couldn't find preview");

    resolution.set_text_content(Some(&message));

    // let image = image.decode().expect("can't decode").grayscale();

    // return Uint8Array::from(image.as_bytes());
}
