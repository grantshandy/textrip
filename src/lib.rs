extern crate image;
extern crate imageproc;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::io::Cursor;

use image::io::Reader as ImageReader;
use js_sys::Uint8Array;
use web_sys::File;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn run(value: &[u8]) -> Uint8Array {
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

    let value: Uint8Array = invert(value).as_slice().into();
    // let file = File::new_with_u8_array_sequence(value, "output.png").unwrap();

    return value;
}

pub fn get_dimensions(value: &[u8]) -> String {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let (x, y) = image.into_dimensions().unwrap();
    format!("resolution: ({}, {})", x, y)
}

#[wasm_bindgen]
pub fn image_width(value: &[u8]) -> String {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let (x, _) = image.into_dimensions().unwrap();

    return x.to_string();
}

#[wasm_bindgen]
pub fn image_height(value: &[u8]) -> String {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let (_, y) = image.into_dimensions().unwrap();

    return y.to_string();
}

pub fn invert(value: &[u8]) -> Vec<u8> {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let mut image = match image.decode() {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    image.invert();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut bytes, image::ImageOutputFormat::Png)
        .expect("Can write to png");

    return bytes;
}
