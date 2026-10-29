use crate::{ray::Ray, vec3::Color};

pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<Hit>;
}

pub struct Hit {
    pub distance: f64,
    pub color: Color,
}