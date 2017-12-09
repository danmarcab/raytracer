extern crate raytracer;
extern crate image;
extern crate serde;
extern crate serde_json;

use image::ImageFormat;
use std::fs::{File,OpenOptions};
use std::io::Read;

use raytracer::camera::Camera;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut file = File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let scene = serde_json::from_str(&data).unwrap();

    let camera = Camera {
        sensor_width: 800,
        sensor_height: 600,
        field_of_view: 45.0,
    };

    let image = raytracer::render(&camera, &scene);

    let mut image_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("render.png")
        .unwrap();

    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
