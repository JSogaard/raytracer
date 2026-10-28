use crate::{hittable::Hittable, ray::Ray, vec3::Point};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> Option<f64> {
        let orig = ray.origin();
        let dir = ray.direction();

        let a = dir.dot(dir);
        let b = -2. * dir.dot(self.center - orig);
        let c = (self.center - orig).dot(self.center - orig) - self.radius * self.radius;
        let d = b * b - 4. * a * c;

        if d < 0. || a == 0. {
            None
        } else {
            let near = (-b - d.sqrt()) / (2. * a);
            let far = (-b + d.sqrt()) / (2. * a);
            
            if near >= 0. {
                Some(near)
            } else if far >= 0. {
                Some(far)
            } else {
                None
            }
        }
    }
}
