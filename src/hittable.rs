use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<HitRecord>;
}

pub struct HitRecord;