use vect3::Vect3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vect3,
    pub direction: Vect3,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub surface_color: Vect3,
    pub position: Vect3,
    pub normal: Vect3,
    pub distance_to_orig: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
