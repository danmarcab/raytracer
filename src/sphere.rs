use ray::{Ray, Hittable, Hit};
use vect3::Vect3;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sphere {
    pub center: Vect3,
    pub radius: f64,
    pub surface_color: Vect3,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let orig_to_center: Vect3 = self.center - ray.origin;

        let orig_proj_dist = orig_to_center.dot(&ray.direction);
        let proj = ray.origin + orig_proj_dist * ray.direction;

        let center_to_proj: Vect3 = proj - self.center;

        let proj_to_center_dist_sq = center_to_proj.length_sq();
        let radius_sq = self.radius * self.radius;

        if proj_to_center_dist_sq < radius_sq {
            let proj_to_hit_dist = (radius_sq - proj_to_center_dist_sq).sqrt();
            let distance_to_orig = orig_proj_dist - proj_to_hit_dist;
            let hit_point = ray.direction * distance_to_orig;
            let normal = hit_point - self.center;

            Some(Hit {
                surface_color: self.surface_color,
                position: hit_point,
                normal: normal.normalize(),
                distance_to_orig: distance_to_orig,
            })
        } else {
            None
        }
    }
}
