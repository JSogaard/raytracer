use crate::shapes::sphere::Sphere;
use crate::vec3::Point;
use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::fmt::Debug;
use std::sync::Arc;

// pub trait Hittable: Send + Sync + Debug {
//     fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit>;
// }

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, ray_t),
        }
    }
}

pub struct Hit {
    pub distance: f64,
    pub normal: Vec3,
    pub hit_point: Point,
    pub material: Arc<dyn Material>,
}
