use crate::vec3::Point;
use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit>;
}

pub struct Hit {
    pub distance: f64,
    pub normal: Vec3,
    pub hit_point: Point,
    pub material: Arc<dyn Material>,
}
