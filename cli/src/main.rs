extern crate raytracer;
extern crate image;

use image::ImageFormat;
use std::fs::OpenOptions;
use raytracer::vect3::Vect3;
use raytracer::sphere::Sphere;
use raytracer::camera::Camera;
use raytracer::scene::{Scene, Object};

fn main() {
    let camera = Camera {
        sensor_width: 800,
        sensor_height: 600,
        field_of_view: 45.0,
    };

    let scene = Scene {
        background_color: Vect3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        objects: [
            Object::Sphere(Sphere {
                center: Vect3 {
                    x: 3.0,
                    y: 3.0,
                    z: -20.0,
                },
                radius: 5.0,
                surface_color: Vect3 {
                    x: 255.0,
                    y: 100.0,
                    z: 100.0,
                },
            }),
            Object::Sphere(Sphere {
                center: Vect3 {
                    x: 0.0,
                    y: 0.0,
                    z: -10.0,
                },
                radius: 1.0,
                surface_color: Vect3 {
                    x: 100.0,
                    y: 255.0,
                    z: 100.0,
                },
            }),
        ].to_vec(),
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
