use ray::{Ray, Hittable, Hit};
use vect3::Vect3;
use sphere::Sphere;
use plane::Plane;
use std;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scene {
    pub background_color: Vect3,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn iluminate_hit(&self, hit: &Hit) -> Vect3 {
        let mut color = Vect3::zero();
        for light in &self.lights {
            let direction_to_light = light.direction_from(&hit.position);

            let light_intensity = light.intensity();
            let light_power = hit.normal.dot(&direction_to_light).max(0.0) * light_intensity;
            let light_reflected = 0.5 / std::f64::consts::PI;
            let light_color = light.color() * light_power * light_reflected;
            color = color + (hit.surface_color * light_color);
        }
        Vect3 {
            x: color.x.max(0.0).min(255.0),
            y: color.y.max(0.0).min(255.0),
            z: color.z.max(0.0).min(255.0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Light {
    Directional(DirectionalLight),
}

impl Light {
    pub fn direction_from(&self, _from: &Vect3) -> Vect3 {
        match self {
            &Light::Directional(ref light) => -light.direction,
        }
    }

    pub fn intensity(&self) -> f64 {
        match self {
            &Light::Directional(ref light) => light.intensity,
        }
    }

    pub fn color(&self) -> Vect3 {
        match self {
            &Light::Directional(ref light) => light.color,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vect3,
    pub color: Vect3,
    pub intensity: f64,
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        match *self {
            Object::Sphere(ref sphere) => sphere.hit(ray),
            Object::Plane(ref plane) => plane.hit(ray),
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
