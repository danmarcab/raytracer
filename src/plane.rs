use ray::{Ray, Hittable, Hit};
use vect3::Vect3;

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vect3,
    pub normal: Vect3,
    pub surface_color: Vect3,
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let proj = self.normal.dot(&ray.direction);

        // ray must point against normal, with a little tolerance
        if proj < 1e-8 {
            let vect_to_ray_orig = ray.origin - self.point;
            let dist = vect_to_ray_orig.dot(&self.normal) / -proj;
            if dist > 0.0 {
                Some(Hit {
                    surface_color: self.surface_color,
                    distance_to_orig: dist,
                })
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}
