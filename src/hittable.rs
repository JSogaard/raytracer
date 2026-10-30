use std::fmt::Debug;

use crate::{interval::Interval, ray::Ray, vec3::Vec3};

pub trait Hittable: Send + Sync + Debug {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit>;
}

pub struct Hit {
    pub distance: f64,
    pub normal: Vec3,
}