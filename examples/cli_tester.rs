extern crate textrip;

use std::{env::args, io::Write, path::Path};
use textrip::invert;

fn main() {
    let image = include_bytes!("test_image.jpg");

    let inverted_image = invert(image);

    let path = Path::new("test_image_inverted.jpg");

    std::fs::write(path, inverted_image.as_slice()).unwrap();
}
