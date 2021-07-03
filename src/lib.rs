extern crate image;
extern crate imageproc;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use std::io::Cursor;
use wasm_bindgen::prelude::*;

use image::io::Reader as ImageReader;
use image::{DynamicImage, Rgba};
use imageproc::geometric_transformations::{warp, Interpolation, Projection};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub fn warp_image(image_bytes: &[u8], c1: Coords, c2: Coords, c3: Coords, c4: Coords) -> Vec<u8> {
    let image = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let x_coords = [c1.x, c2.x, c3.x, c4.x];
    let y_coords = [c1.y, c2.y, c3.y, c4.y];
    let min_x = *x_coords.iter().min().unwrap();
    let max_x = *x_coords.iter().max().unwrap();
    let min_y = *y_coords.iter().min().unwrap();
    let max_y = *y_coords.iter().max().unwrap();
    let bounding_box_width = max_x as f32 - min_x as f32;
    let bounding_box_height = max_y as f32 - min_y as f32;

    log(&format!(
        "bounding_box w/h ({}, {})",
        bounding_box_width, bounding_box_height
    ));

    let top_dist = compute_distance(c1, c2);
    let bottom_dist = compute_distance(c3, c4);
    let left_dist = compute_distance(c1, c3);
    let right_dist = compute_distance(c2, c4);
    log(&format!("top/bottom dist ({}, {})", top_dist, bottom_dist));
    log(&format!("left/right dist ({}, {})", left_dist, right_dist));

    let width_scale = left_dist.max(right_dist) / left_dist.min(right_dist);
    let height_scale = top_dist.max(bottom_dist) / top_dist.min(bottom_dist);
    log(&format!(
        "width_scale: {}, height_scale: {}",
        width_scale, height_scale
    ));

    let projection = Projection::from_control_points(
        // [top-left, top-right, bottom-left, bottom-right]
        [
            (c1.x as f32, c1.y as f32),
            (c2.x as f32, c2.y as f32),
            (c3.x as f32, c3.y as f32),
            (c4.x as f32, c4.y as f32),
        ],
        [
            (min_x as f32, min_y as f32),
            (max_x as f32, min_y as f32),
            (min_x as f32, max_y as f32),
            (max_x as f32, max_y as f32),
        ],
    )
    .unwrap();

    let image_buffer = image.into_rgba8();
    let warped_image_buffer = warp(
        &image_buffer,
        &projection,
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
    );
    // let cropped_image = warped_image.crop_imm(
    //     min_x,
    //     min_y,
    //     bounding_box_width as u32,
    //     bounding_box_height as u32,
    // );
    let projection = Projection::scale(width_scale, height_scale);
    let scaled_image_buffer = warp(
        &warped_image_buffer,
        &projection,
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
    );
    let scaled_image = DynamicImage::ImageRgba8(scaled_image_buffer);
    let crop_x = ((min_x as f32) * width_scale) as u32;
    let crop_y = ((min_y as f32) * height_scale) as u32;
    let crop_width = (bounding_box_width * width_scale) as u32;
    let crop_height = (bounding_box_height * height_scale) as u32;
    let cropped_image = scaled_image.crop_imm(crop_x, crop_y, crop_width, crop_height);
    let mut bytes: Vec<u8> = Vec::new();
    cropped_image
        .write_to(&mut bytes, image::ImageOutputFormat::Png)
        .expect("Can write to png");
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
#[derive(Clone, Copy)]
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

fn compute_distance(c1: Coords, c2: Coords) -> f32 {
    let x_diff = c1.x as f32 - c2.x as f32;
    let y_diff = c1.y as f32 - c2.y as f32;
    (x_diff * x_diff + y_diff * y_diff).sqrt()
}
