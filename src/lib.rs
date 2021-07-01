extern crate image;
extern crate imageproc;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::io::Cursor;

use image::io::Reader as ImageReader;
use js_sys::Uint8Array;
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
    let dimensions = get_dimensions(value);

    let x = dimensions.width;
    let y = dimensions.height;

    let message = format!("({}, {})", x, y);

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

#[wasm_bindgen]
pub fn get_dimensions(value: &[u8]) -> Resolution {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let (width, height) = image.into_dimensions().unwrap();
    
    return Resolution { width, height };
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

#[wasm_bindgen]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}