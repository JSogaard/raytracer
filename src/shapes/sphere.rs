use crate::{ hittable::{Hit, Hittable}, interval::Interval, ray::Ray, vec3::{Point, Vec3}};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    fn normal(&self, ray: &Ray, distance: f64) -> Vec3 {
        let intersept: Point = ray.point_at(distance);
        let normal = (intersept - self.center) / self.radius;
        if ray.direction().dot(normal) <= 0.0 {
            normal
        } else {
            -normal
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
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
                Some(Hit {
                    distance: near,
                    normal: self.normal(ray, near),
                })
            } else if ray_t.surrounds(far) {
                Some(Hit {
                    distance: far,
                    normal: self.normal(ray, far),
                })
            } else {
                None
            }
        }
    }
}
