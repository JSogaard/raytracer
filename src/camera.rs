use crate::{
    interval::Interval, ray::Ray, scene::Scene, vec3::{Color, Point, Vec3},
};
use image::RgbImage;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone)]
pub struct Camera {
    pixel_width: u32,
    pixel_height: u32,
    center: Point,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
    top_left: Point,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        pixel_height: u32,
        vfov: f64,
        center: Point,
        direction: Vec3,
        up_direction: Vec3,
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

        Self {
            pixel_width,
            pixel_height,
            center,
            pixel_delta_x,
            pixel_delta_y,
            top_left,
        }
    }

    pub fn render(&self, scene: &Scene, ray_t: Interval) -> RgbImage {
        let mut image: RgbImage = RgbImage::new(self.pixel_width, self.pixel_height);

        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let pixel_vec =
                self.top_left + x as f64 * self.pixel_delta_x + y as f64 * self.pixel_delta_y;
            let direction = pixel_vec - self.center;
            let ray = Ray::new(self.center, direction);

            let hit = scene.hit(&ray, ray_t);

            let color: Color = if let Some(hit) = hit {
                calc_color(hit.normal)
            } else {
                let a = (&ray.direction().z() + 1.0) / 2.0;
                a * Color::new(0.5, 0.7, 1.) + (1. - a) * Color::new(1.0, 1.0, 1.0)
            };

            *pixel = color.to_rgb();
        });

        image
    }
}

fn calc_color(normal: Vec3) -> Color {
    let r = (normal.x() + 1.0) / 2.0;
    let g = (normal.z() + 1.0) / 2.0;
    let b = (normal.y() + 1.0) / 2.0;

    Color::new(r, g, b)
}
