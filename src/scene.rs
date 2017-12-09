use ray::{Ray, Hittable, Hit};
use vect3::Vect3;
use sphere::Sphere;

pub struct Scene {
    pub background_color: Vect3,
    pub objects: Vec<Object>,
}

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        match *self {
            Object::Sphere(ref sphere) => sphere.hit(ray),
        }
    }
}


impl Hittable for Scene {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.objects.iter().filter_map(|obj| obj.hit(ray)).min_by(
            |i1,
             i2| {
                i1.distance_to_orig
                    .partial_cmp(&i2.distance_to_orig)
                    .unwrap()
            },
        )
    }
}