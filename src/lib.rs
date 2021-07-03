extern crate image;
extern crate imageproc;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use std::io::Cursor;

use imageproc::geometric_transformations::{Projection, warp, Interpolation};
use image::{Rgb, DynamicImage};
use image::io::Reader as ImageReader;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn warp_image(image_bytes: &[u8], c1: Coords, c2: Coords, c3: Coords, c4: Coords) -> Vec<u8> {
    let image = match ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format() {
            Ok(data) => data,
            Err(e) => {
                log(&format!("{}", e));
                panic!("");
            }
        };


    let image = match image.decode() {
            Ok(data) => data,
            Err(e) => {
                log(&format!("{}", e));
                panic!("");
            }
        };

    let x_coords = [c1.x, c2.x, c3.x, c4.x];
    let y_coords = [c1.y, c2.y, c3.y, c4.y];
    let min_x = x_coords.iter().min().unwrap();
    let max_x = x_coords.iter().max().unwrap();
    let min_y = y_coords.iter().min().unwrap();
    let max_y = y_coords.iter().max().unwrap();

    let width = max_x - min_x;
    let height = max_y - min_y;
    let cropped_image = image.crop_imm(*min_x, *min_y, width, height);

    let image_buffer = match cropped_image.as_rgb8() {
        Some(data) => data,
        None => {
            log("error creating the buffer");
            panic!("");
        }
    };

    log(&format!("from ({}, {}) ({}, {}) ({}, {}) ({}, {})", c1.x, c1.y, c2.x, c2.y, c3.x, c3.y, c4.x, c4.y));
    log(&format!("to ({}, {}) ({}, {}) ({}, {}) ({}, {})", min_x, max_y, max_x, max_y, max_x, min_y, min_x, min_x));


    let projection = match Projection::from_control_points(
        [(c1.x as f32, c1.y as f32), (c2.x as f32, c2.y as f32), (c3.x as f32, c3.y as f32), (c4.x as f32, c4.y as f32)],
        [(*min_x as f32, *max_y as f32), (*max_x as f32, *max_y as f32), (*max_x as f32, *min_y as f32), (*min_x as f32, *min_x as f32)]
    ) {
        Some(data) => data,
        None => {
            log("error creating the Projection");
            panic!("");
        }
    };

    let warped_image = warp(
        &image_buffer,
        &projection,
        Interpolation::Nearest,
        Rgb([0, 0, 0])
    );

    // let warped_image = DynamicImage::ImageRgb8(warped_image);

    let mut bytes: Vec<u8> = Vec::new();
    // warped_image.write_to(&mut bytes, image::ImageOutputFormat::Png).expect("Can write to png");
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
pub struct Resolution {
    pub width: u32,
    pub height: u32,
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