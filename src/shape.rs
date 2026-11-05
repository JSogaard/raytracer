use crate::shapes::sphere::Sphere;
use crate::vec3::Point;
use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, ray_t),
        }
    }
}

pub struct Hit<'a> {
    pub distance: f64,
    pub normal: Vec3,
    pub hit_point: Point,
    pub material: &'a dyn Material,
    pub front_face: bool,
}
