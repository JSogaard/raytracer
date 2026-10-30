use crate::{ hittable::{Hit, Hittable}, ray::Ray, vec3::{Color, Point}};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    fn calc_color(&self, ray: &Ray, distance: f64) -> Color {
        let intersept: Point = ray.point_at(distance);
        let normal = (intersept - self.center).normalize();

        let r = (normal.z() + 1.0) / 2.0;
        let g = (normal.x() + 1.0) / 2.0;
        let b = (normal.y() + 1.0) / 2.0;

        Color::new(r, g, b)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> Option<Hit> {
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
            
            if near >= 0. {
                Some(Hit {
                    distance: near,
                    color: self.calc_color(&ray, near),
                })
            } else if far >= 0. {
                Some(Hit {
                    distance: far,
                    color: self.calc_color(&ray, far),
                })
            } else {
                None
            }
        }
    }
}
