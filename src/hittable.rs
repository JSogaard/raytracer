use crate::{ray::Ray, vec3::Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub struct Hit {
    pub distance: f64,
    pub normal: Vec3,
}