
use crate::hittable::Hit;
use crate::material::{Material, Scatter};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Debug, Copy, Clone)]
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
