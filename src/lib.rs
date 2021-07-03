extern crate image;
extern crate imageproc;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::io::Cursor;

use image::io::Reader as ImageReader;
use js_sys::Uint8Array;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}

#[wasm_bindgen]
impl Coords {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Coords {
        Coords { x, y }
    }
}

#[wasm_bindgen]
pub fn crop_image(image_bytes: &[u8], c1: Coords, c2: Coords, c3: Coords, c4: Coords) -> Vec<u8> {
    let image = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .expect("can't get imagereader from the image")
        .decode()
        .expect("can't decode image");

    let x_coords = [c1.x, c2.x, c3.x, c4.x];
    let y_coords = [c1.y, c2.y, c3.y, c4.y];
    let min_x = x_coords.iter().min().unwrap();
    let max_x = x_coords.iter().max().unwrap();
    let min_y = y_coords.iter().min().unwrap();
    let max_y = y_coords.iter().max().unwrap();
    log(&format!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y));

    let width = max_x - min_x;
    let height = max_y - min_y;
    let cropped_image = image.crop_imm(*min_x, *min_y, width, height);
    let mut bytes: Vec<u8> = Vec::new();
    cropped_image.write_to(&mut bytes, image::ImageOutputFormat::Png).expect("Can write to png");
    bytes
}

#[wasm_bindgen]
pub fn get_dimensions(value: &[u8]) -> Resolution {
    let image = ImageReader::new(Cursor::new(value))
        .with_guessed_format()
        .expect("can't get imagereader from the image");

    let (width, height) = image.into_dimensions().unwrap();

    return Resolution { width, height };
}

#[wasm_bindgen]
pub fn print_points(value: Array) {
    let value = value.to_vec();

    for x in value {
        let array = Array::from(&x);


    }
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
