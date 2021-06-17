#![feature(proc_macro)]

extern crate image;
extern crate wasm_bindgen;

use std::fs::File;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate stdweb;

use stdweb::js_export;
use stdweb::web::FileReader;
use stdweb::web::FileReaderResult;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[js_export]
pub fn resolution(file_reader: FileReader) -> String {
    match file_reader.result() {
        Some(data) => match data {
            FileReaderResult::ArrayBuffer(value) => {
                // let image = ImageReader::new(Cursor::new(value));

                // let (x, y) = match image.into_dimensions() {
                //     Ok(data) => data,
                //     Err(e) => {
                //         let e = format!("image-rs error: {}", e);
                //         error(&e);
                //         return e;
                //     },
                // };
            
                // return format!("image-rs: {}, {}", x, y);

                return "got an array buffer".to_string();
            }
            _ => return "bad input".to_string(),
        }
        None => return "no input".to_string(),
    };
}
