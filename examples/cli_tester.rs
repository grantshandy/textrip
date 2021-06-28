extern crate textrip;

use std::{env::args, io::Write, path::Path};
use textrip::invert;

fn main() {
    let image = include_bytes!("test_image.jpg");

    let x = invert(image);

    let path = Path::new("/home/grant/Pictures/output.jpg");

    std::fs::write(path, image);
}