use std::fmt::Debug;

use crate::{shape::Hit, ray::Ray, vec3::Color};

pub trait Material:Send + Sync + Debug {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}