use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>>;
    fn xs(&self) -> Interval;
    fn ys(&self) -> Interval;
    fn zs(&self) -> Interval;
    fn center(&self) -> Vec3;
}

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    pub distance: f64,
    pub normal: Vec3,
    pub hit_point: Vec3,
    pub material: &'a Material,
    pub front_face: bool,
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    xs: Interval,
    ys: Interval,
    zs: Interval,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<Material>) -> Self {
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

    fn normal(&self, ray: &Ray, intercept: Vec3) -> (Vec3, bool) {
        let normal = (intercept - self.center) / self.radius;
        if ray.direction().dot(normal) <= 0.0 {
            (normal, true)
        } else {
            (-normal, false)
        }
    }
}

impl Hittable for Sphere {
        fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
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

            let t = if ray_t.contains(near) {
                near
            } else if ray_t.contains(far) {
                far
            } else {
                return None;
            };

            let hit_point: Vec3 = ray.point_at(t);
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

    fn xs(&self) -> Interval {
        self.xs
    }

    fn ys(&self) -> Interval {
        self.ys
    }

    fn zs(&self) -> Interval {
        self.zs
    }

    fn center(&self) -> Vec3 {
        self.center
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Quad {
    orig: Vec3,
    vert1: Vec3,
    vert2: Vec3,
    normal: Vec3,
    d: f64,
    xs: Interval,
    ys: Interval,
    zs: Interval,
    mat: Arc<Material>,
}

impl Quad {
    pub fn new(orig: Vec3, vert1: Vec3, vert2: Vec3, mat: Arc<Material>) -> Self {
        let normal = vert1.cross(vert2).normalize();
        let d = normal.dot(orig);
        let opposit = orig + vert1 + vert2;

        let points = [orig, orig + vert1, orig + vert2, opposit];

        let interval_for = |axis: fn(&Vec3) -> f64| {
            let min = points
                .iter()
                .map(axis)
                .min_by(|a, b| a.total_cmp(b))
                .unwrap();
            let max = points
                .iter()
                .map(axis)
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();
            Interval::new(min, max)
        };

        let (xs, ys, zs) = (
            interval_for(Vec3::x),
            interval_for(Vec3::y),
            interval_for(Vec3::z),
        );

        Self {
            orig,
            vert1,
            vert2,
            normal,
            d,
            xs,
            ys,
            zs,
            mat,
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
}
