use crate::{ray::Ray, vec3::Color};

pub trait Hittable {
    fn hit(&self, ray: Ray) -> HitResult;
}

pub enum HitResult {
    Miss,
    Hit {
        distance: f64,
        color: Color,
    }
}