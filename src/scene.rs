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

            let ray_to_light = Ray {
                origin: (*hit).position + 0.001 * (*hit).normal,
                direction: direction_to_light,
            };

            match self.hit(&ray_to_light) {
                Some(light_hit) => {
                    if light_hit.distance_to_orig > light.distance_from(&hit.position) {
                        color = color + light_contribution(&light, &hit, &direction_to_light);
                    }
                }
                None => {
                    color = color + light_contribution(&light, &hit, &direction_to_light);
                }
            }

        }
        Vect3 {
            x: color.x.max(0.0).min(255.0),
            y: color.y.max(0.0).min(255.0),
            z: color.z.max(0.0).min(255.0),
        }
    }
}

fn light_contribution(light: &Light, hit: &Hit, direction_to_light: &Vect3) -> Vect3 {
    let light_intensity = light.intensity(&hit.position);
    let light_power = hit.normal.dot(&direction_to_light).max(0.0) * light_intensity;
    let light_reflected = 0.5 / std::f64::consts::PI;
    let light_color = light.color() * light_power * light_reflected;

    hit.surface_color * light_color
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
}

impl Light {
    pub fn direction_from(&self, from: &Vect3) -> Vect3 {
        match self {
            &Light::Directional(ref light) => -light.direction.normalize(),
            &Light::Point(ref light) => (light.position - *from).normalize(),
        }
    }

    pub fn distance_from(&self, from: &Vect3) -> f64 {
        match self {
            &Light::Directional(ref _light) => ::std::f64::INFINITY,
            &Light::Point(ref light) => (light.position - *from).length(),
        }
    }

    pub fn intensity(&self, from: &Vect3) -> f64 {
        match self {
            &Light::Directional(ref light) => light.intensity,
            &Light::Point(ref light) => {
                let dist_sq = (light.position - *from).length_sq() as f64;
                light.intensity / (4.0 * ::std::f64::consts::PI * dist_sq)

            }
        }
    }

    pub fn color(&self) -> Vect3 {
        match self {
            &Light::Directional(ref light) => light.color,
            &Light::Point(ref light) => light.color,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DirectionalLight {
    pub direction: Vect3,
    pub color: Vect3,
    pub intensity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PointLight {
    pub position: Vect3,
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
