extern crate image;
extern crate wasm_bindgen;

// use std::fs::File;
use image::io::Reader as ImageReader;
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

    let val = document.create_element("p").expect("couldn't create p element");
    val.set_text_content(Some("Hello from Rust!"));
    body.append_child(&val).expect("couldn't append child");
}
