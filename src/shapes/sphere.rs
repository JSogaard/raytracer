use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    shape::Hit,
    vec3::{Point, Vec3},
};
use std::sync::Arc;

#[derive(Debug, Clone)] 
pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        let orig = ray.origin();
        let dir = ray.direction();

        let a = dir.dot(dir);
        let h = dir.dot(self.center - orig);
        let c = (self.center - orig).dot(self.center - orig) - self.radius * self.radius;
        let d = h * h - a * c;

        if d < 0. || a == 0. {
            None
        } else {
            let near = (h - d.sqrt()) / a;
            let far = (h + d.sqrt()) / a;

            if ray_t.surrounds(near) {
                let intercept: Point = ray.point_at(near);
                Some(Hit {
                    distance: near,
                    normal: self.normal(ray, intercept),
                    hit_point: ray.point_at(near),
                    material: self.material.clone(),
                })
            } else if ray_t.surrounds(far) {
                let intercept: Point = ray.point_at(far);
                Some(Hit {
                    distance: far,
                    normal: self.normal(ray, intercept),
                    hit_point: intercept,
                    material: self.material.clone(),
                })
            } else {
                None
            }
        }
    }

    fn normal(&self, ray: &Ray, intercept: Point) -> Vec3 {
        let normal = (intercept - self.center) / self.radius;
        if ray.direction().dot(normal) <= 0.0 {
            normal
        } else {
            -normal
        }
    }
}
