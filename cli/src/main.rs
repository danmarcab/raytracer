extern crate raytracer;
extern crate image;

use image::ImageFormat;
use std::fs::OpenOptions;

fn main() {
    let image = raytracer::render();
    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("render.png")
        .unwrap();

    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
