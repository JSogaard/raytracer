use crate::{hittable::{HitResult::{self, Hit, Miss}, Hittable}, ray::Ray, vec3::{Color, Point}};

pub struct Sphere {
    center: Point,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, color: Color) -> Self {
        Self { center, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> HitResult {
        let orig = ray.origin();
        let dir = ray.direction();

        let a = dir.dot(dir);
        let b = -2. * dir.dot(self.center - orig);
        let c = (self.center - orig).dot(self.center - orig) - self.radius * self.radius;
        let d = b * b - 4. * a * c;

        if d < 0. || a == 0. {
            Miss
        } else {
            let near = (-b - d.sqrt()) / (2. * a);
            let far = (-b + d.sqrt()) / (2. * a);
            
            if near >= 0. {
                Hit {
                    distance: near,
                    color: self.color,
                }
            } else if far >= 0. {
                Hit {
                    distance: far,
                    color: self.color,
                }
            } else {
                Miss
            }
        }
    }
}
