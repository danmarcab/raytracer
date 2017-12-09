use image::{DynamicImage, GenericImage, Pixel, Rgba};
use vect3::Vect3;
use ray::{Ray, Hittable};
use scene::Scene;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Camera {
    pub sensor_width: u32,
    pub sensor_height: u32,
    pub field_of_view: f64,

    // TODO: de-harcode camera position and angle
    // pub position: Vect3,
    // pub direction: Vect3,
}

impl Camera {
    pub fn render(&self, scene: &Scene) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.sensor_width, self.sensor_height);

        for x in 0..self.sensor_width {
            for y in 0..self.sensor_height {
                let ray = self.trace_ray(x, y);

                match scene.hit(&ray) {
                    Some(hit) => image.put_pixel(x, y, to_rgba(&hit.surface_color)),
                    None => image.put_pixel(x, y, to_rgba(&scene.background_color)),
                }
            }
        }
        image
    }

    fn trace_ray(&self, x: u32, y: u32) -> Ray {
        let coord_x = ((x as f64 + 0.5) / self.sensor_width as f64) * 2.0 - 1.0;
        let coord_y = 1.0 - ((y as f64 + 0.5) / self.sensor_height as f64) * 2.0;

        let aspect_adjustment = (self.sensor_width as f64) / (self.sensor_height as f64);
        let field_of_view_adjustment = (self.field_of_view.to_radians() / 2.0).tan();

        // TODO: de-harcode camera position and angle
        Ray {
            origin: Vect3::zero(),
            direction: Vect3 {
                x: coord_x * aspect_adjustment * field_of_view_adjustment,
                y: coord_y * field_of_view_adjustment,
                z: -1.0,
            }.normalize(),
        }
    }
}


fn to_rgba(color: &Vect3) -> Rgba<u8> {
    Rgba::from_channels(color.x as u8, color.y as u8, color.z as u8, 0)
}
