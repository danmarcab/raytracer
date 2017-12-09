extern crate image;

pub mod vect3;
pub mod ray;
pub mod camera;
pub mod scene;
pub mod sphere;
pub mod plane;

use image::DynamicImage;
use camera::Camera;
use scene::Scene;

pub fn render(camera: &Camera, scene: &Scene) -> DynamicImage {
    camera.render(scene)
}
