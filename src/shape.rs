use crate::interval::{self, INF_INTERVAL};
use crate::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>>;
    fn xs(&self) -> Interval;
    fn ys(&self) -> Interval;
    fn zs(&self) -> Interval;
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

    pub fn center(&self) -> Vec3 {
        self.center
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
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Quad {
    orig: Vec3,
    vert1: Vec3,
    vert2: Vec3,
    normal: Vec3,
    w: Vec3,
    d: f64,
    xs: Interval,
    ys: Interval,
    zs: Interval,
    mat: Arc<Material>,
}

impl Quad {
    pub fn new(orig: Vec3, vert1: Vec3, vert2: Vec3, mat: Arc<Material>) -> Self {
        let n = vert1.cross(vert2);
        let normal = n.normalize();
        let d = normal.dot(orig);
        let w = n / n.dot(n);
        let opposit = orig + vert1 + vert2;

        let points = [orig, orig + vert1, orig + vert2, opposit];

        let (xs, ys, zs) = (
            interval_for(&points, Vec3::x),
            interval_for(&points, Vec3::y),
            interval_for(&points, Vec3::z),
        );

        Self {
            orig,
            vert1,
            vert2,
            normal,
            w,
            d,
            xs,
            ys,
            zs,
            mat,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
        let denominator = self.normal.dot(ray.direction());
        if denominator == 0.0 {
            return None;
        }

        let distance = (self.d - self.normal.dot(ray.origin())) / (denominator);
        if !ray_t.contains(distance) {
            return None;
        }

        let hit_point = ray.point_at(distance);
        let rel_hit_point = hit_point - self.orig;
        let front_face = ray.direction().dot(self.normal) < 0.0;
        let normal = if front_face {
            self.normal
        } else {
            -self.normal
        };

        let alpha = self.w.dot(rel_hit_point.cross(self.vert2));
        let beta = self.w.dot(self.vert1.cross(rel_hit_point));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) {
            Some(Hit {
                distance,
                normal,
                hit_point,
                material: &self.mat,
                front_face,
            })
        } else {
            None
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
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Cube {
    quads: [Quad; 6],
    xs: Interval,
    ys: Interval,
    zs: Interval,
}

impl Cube {
    pub fn new(orig: Vec3, u: Vec3, v: Vec3, mat: Arc<Material>) -> Self {
        let v = v - v.dot(u) / u.length_squared() * u;
        let w = u.cross(v);

        let quads = [
            Quad::new(orig, v, u, mat.clone()),
            Quad::new(orig, u, w, mat.clone()),
            Quad::new(orig, w, v, mat.clone()),
            Quad::new(orig + u, v, w, mat.clone()),
            Quad::new(orig + v, w, u, mat.clone()),
            Quad::new(orig + w, u, v, mat),
        ];

        let points = [
            orig,
            orig + u,
            orig + v,
            orig + w,
            orig + u + v,
            orig + u + w,
            orig + v + w,
            orig + u + v + w,
        ];

        let (xs, ys, zs) = (
            interval_for(&points, Vec3::x),
            interval_for(&points, Vec3::y),
            interval_for(&points, Vec3::z),
        );

        Self {
            quads,
            xs,
            ys,
            zs,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
        let (closest_hit, _) = self.quads.iter().fold(
            (None, f64::INFINITY),
            |(closest_hit, closest), quad| match quad.hit(ray, ray_t) {
                Some(hit) if hit.distance < closest => {
                    let distance = hit.distance;
                    (Some(hit), distance)
                }
                _ => (closest_hit, closest),
            },
        );

        closest_hit
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
}

//------------------------------------------------------------------------------

fn interval_for(points: &[Vec3], axis: fn(&Vec3) -> f64) -> Interval {
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
}
