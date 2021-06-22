extern crate textrip;

use textrip::get_dimensions;

fn main() {
    let image = include_bytes!("test_image.jpg");

    let x = get_dimensions(image);
    println!("{}", x);
}