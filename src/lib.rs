extern crate image;
extern crate wasm_bindgen;

// use std::fs::File;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use wasm_bindgen::prelude::*;

// #[macro_use]
// extern crate stdweb;

// use stdweb::*;
// use stdweb::web::FileReader;
// use stdweb::web::FileReaderResult;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn resolution(value: &[u8]) -> String {
    log(&format!("bytes: {}", &value.len()));

    let image = match ImageReader::new(Cursor::new(value))
        .with_guessed_format() {
            Ok(data) => data,
            Err(e) => {
                let e = format!("image-rs error: {}", e);
                error(&e);
                return e;
            }
        };

    let (x, y) = match image.into_dimensions() {
        Ok(data) => data,
        Err(e) => {
            let e = format!("image-rs error: {}", e);
            error(&e);
            return e;
        }
    };

    return format!("image-rs: {}, {}", x, y);
}
