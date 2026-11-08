use crate::vec3::Point;
use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::fmt::Debug;
use std::sync::Arc;

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

    pub fn xs(&self) -> Interval {
        match self {
            Shape::Sphere(sphere) => sphere.xs(),
        }
    }

    pub fn ys(&self) -> Interval {
        match self {
            Shape::Sphere(sphere) => sphere.ys(),
        }
    }

    pub fn zs(&self) -> Interval {
        match self {
            Shape::Sphere(sphere) => sphere.zs(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    pub distance: f64,
    pub normal: Vec3,
    pub hit_point: Point,
    pub material: &'a Material,
    pub front_face: bool,
}



//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    xs: Interval,
    ys: Interval,
    zs: Interval,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<Material>) -> Self {
        let xs = Interval::new(center.x() - radius, center.x() + radius);
        let ys = Interval::new(center.y() - radius, center.y() + radius);
        let zs = Interval::new(center.z() - radius, center.z() + radius);

        Self {
            center,
            radius,
            xs,
            ys,
            zs,
            material,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
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

            let t = if ray_t.surrounds(near) {
                near
            } else if ray_t.surrounds(far) {
                far
            } else {
                return None;
            };

            let hit_point: Point = ray.point_at(t);
            let (normal, front_face) = self.normal(ray, hit_point);
            Some(Hit {
                distance: t,
                normal,
                hit_point,
                material: self.material.as_ref(),
                front_face,
            })
        }
    }

    pub fn xs(&self) -> Interval {
        self.xs
    }

    pub fn ys(&self) -> Interval {
        self.ys
    }

    pub fn zs(&self) -> Interval {
        self.zs
    }

    fn normal(&self, ray: &Ray, intercept: Point) -> (Vec3, bool) {
        let normal = (intercept - self.center) / self.radius;
        if ray.direction().dot(normal) <= 0.0 {
            (normal, true)
        } else {
            (-normal, false)
        }
    }
}
