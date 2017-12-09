extern crate image;

use image::DynamicImage;

pub fn render() -> DynamicImage {
    let image = DynamicImage::new_rgb8(800, 600);

    image
}
