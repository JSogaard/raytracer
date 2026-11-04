use core::panic;
use std::fmt::Debug;

use crate::{
    ray::Ray,
    shape::Hit,
    vec3::{Color, Vec3},
};

pub trait Material: Send + Sync + Debug {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

/// Lambertian diffuse material
#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut direction = hit.normal + Vec3::random_unit_vec();
        if direction.near_zero() {
            direction = hit.normal
        }
        let new_ray = Ray::new(hit.hit_point, direction);
        Some(Scatter {
            ray: new_ray,
            attenuation: self.albedo,
        })
    }
}

/// Polished metal material
#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz >= 1.0 {
            panic!("Fuzziness factor has to be less than 1.0");
        }
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut direction =
            ray_in.direction().reflect(hit.normal);
        if direction.dot(hit.normal) < 0.0 {
            return None;
        }
        direction += Vec3::random_unit_vec() * self.fuzz;
        let ray = Ray::new(hit.hit_point, direction.normalize());
        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}
