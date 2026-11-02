use crate::{
    interval::Interval,
    ray::Ray,
    scene::Scene,
    vec3::{Color, Point, Vec3},
};
use image::RgbImage;
use rand::random;
use rayon::iter::ParallelIterator;

const MAX_DEPTH: i32 = 10;

#[derive(Debug, Clone)]
pub struct Camera {
    pixel_width: u32,
    pixel_height: u32,
    center: Point,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
    top_left: Point,
    samples_pr_pixel: u32,
    pixel_sample_scale: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        pixel_height: u32,
        vfov: f64,
        center: Point,
        direction: Vec3,
        up_direction: Vec3,
        samples_pr_pixel: u32,
    ) -> Self {
        let vfov = vfov.to_radians();
        let pixel_width = (pixel_height as f64 * aspect_ratio) as u32;
        let viewport_height = 2.0 * (vfov / 2.0).tan();
        let viewport_width = viewport_height * (pixel_width as f64 / pixel_height as f64);

        let direction = direction.normalize();
        let up_direction = up_direction.normalize();

        assert!(
            direction.dot(up_direction).abs() < 1e-10,
            "Camera direction and up-direction are not perpendicular"
        );

        let pixel_delta_x =
            (direction.cross(up_direction)).normalize() * viewport_width / pixel_width as f64;

        let pixel_delta_y = -up_direction * viewport_height / pixel_height as f64;

        let top_left = center + direction - viewport_width / 2. * pixel_delta_x.normalize()
            + viewport_height / 2. * up_direction
            + pixel_delta_x / 2.
            + pixel_delta_y / 2.;

        let pixel_sample_scale = 1.0 / samples_pr_pixel as f64;

        Self {
            pixel_width,
            pixel_height,
            center,
            pixel_delta_x,
            pixel_delta_y,
            top_left,
            samples_pr_pixel,
            pixel_sample_scale,
        }
    }

    pub fn render(&self, scene: &Scene, ray_t: Interval) -> RgbImage {
        let mut image: RgbImage = RgbImage::new(self.pixel_width, self.pixel_height);

        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let pixel_vec =
                self.top_left + x as f64 * self.pixel_delta_x + y as f64 * self.pixel_delta_y;
            let pixel_dir = pixel_vec - self.center;

            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..self.samples_pr_pixel {
                let ray_dir: Vec3 = pixel_dir
                    + (random::<f64>() - 0.5) * self.pixel_delta_x
                    + (random::<f64>() - 0.5) * self.pixel_delta_y;
                let ray = Ray::new(self.center, ray_dir);
                color += ray_color(&ray, ray_t, scene, MAX_DEPTH);
            }

            color *= self.pixel_sample_scale;

            // Clamp and sqrt are conversion from linear to gamma
            *pixel = color.map(|x| x.max(0.0).sqrt()).to_rgb();
        });

        image
    }
}

fn ray_color(ray: &Ray, ray_t: Interval, scene: &Scene, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit = scene.hit(ray, ray_t);

    if let Some(hit) = hit {
        // let hit_point = ray.point_at(hit.distance);
        // let direction = hit.normal + Vec3::random_unit_vec();
        // let new_ray = Ray::new(hit_point, direction);
        let scatter = hit.material.scatter(ray, &hit);
        if let Some(scatter) = scatter {
            scatter.attenuation * ray_color(&scatter.ray, ray_t, scene, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        background_color(ray)
    }
}

fn background_color(ray: &Ray) -> Color {
    let a = (&ray.direction().z() + 1.0) / 2.0;
    a * Color::new(0.4, 0.7, 1.) + (1. - a) * Color::new(1.0, 1.0, 1.0)
}
