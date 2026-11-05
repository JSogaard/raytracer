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
        let mut direction = ray_in.direction().reflect(hit.normal);
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

#[derive(Debug, Clone)]
pub struct Refractive {
    refract_ratio: f64,
}

impl Refractive {
    pub fn new(refract_ratio: f64) -> Self {
        Self { refract_ratio }
    }
}

impl Material for Refractive {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let refract_ratio = if hit.front_face {
            1.0 / self.refract_ratio
        } else {
            self.refract_ratio
        };

        let cos_theta = (-ray_in.direction()).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refract_ratio * sin_theta > 1.0;

        let new_dir = if cannot_refract || reflectance(refract_ratio, cos_theta) > rand::random() {
            ray_in.direction().reflect(hit.normal)
        } else {
            ray_in.direction().refract(hit.normal, refract_ratio)
        };
        let new_ray = Ray::new(hit.hit_point, new_dir);

        Some(Scatter {
            ray: new_ray,
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}

fn reflectance(refract_ratio: f64, cos_theta: f64) -> f64 {
    let r0 = (1.0 - refract_ratio) / (1.0 + refract_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
