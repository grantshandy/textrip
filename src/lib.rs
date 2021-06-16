extern crate image;
extern crate wasm_bindgen;

use std::io::Cursor;

// use std::io::{BufReader, Read};
use image::io::Reader as ImageReader;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn resolution(bytes: &[u8]) -> String {
    log(&format!("rust parameter byte length: {}", &bytes.len()));

    let image = ImageReader::new(Cursor::new(bytes));

    let (x, y) = match image.into_dimensions() {
        Ok(data) => data,
        Err(e) => {
            let e = format!("image-rs error: {}", e);
            error(&e);
            return e;
        },
    };

    return format!("image-rs: {}, {}", x, y);
}
