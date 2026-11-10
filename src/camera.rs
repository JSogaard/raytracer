use crate::{
    interval::Interval,
    ray::Ray,
    scene::Scene,
    vec3::{Color, Vec3},
};
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};

const MAX_DEPTH: i32 = 10;

#[derive(Debug, Clone)]
pub struct Camera {
    pixel_width: u32,
    pixel_height: u32,
    center: Vec3,
    pixel_delta_x: Vec3,
    pixel_delta_y: Vec3,
    top_left: Vec3,
    focus_dist: f64,
    aperture_angle: f64,
}

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub pixel_height: u32,
    pub vfov: f64,
    pub center: Vec3,
    pub direction: Vec3,
    pub up_direction: Vec3,
    pub focus_dist: f64,
    pub aperture_angle: f64,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        if config.vfov <= 0.0 || config.focus_dist <= 0.0 {
            panic!("Focus distance and vertical fov must be positive and non-zero.")
        }

        let vfov = config.vfov.to_radians();
        let pixel_width = (config.pixel_height as f64 * config.aspect_ratio) as u32;
        let viewport_height = 2.0 * config.focus_dist * (vfov / 2.0).tan();
        let viewport_width = viewport_height * (pixel_width as f64 / config.pixel_height as f64);

        let direction = config.direction.normalize();
        let up_direction =
            (config.up_direction - config.up_direction.dot(direction) * direction).normalize();

        let pixel_delta_x =
            (direction.cross(up_direction)).normalize() * viewport_width / pixel_width as f64;

        let pixel_delta_y = -up_direction * viewport_height / config.pixel_height as f64;

        let top_left = config.center + config.focus_dist * direction
            - viewport_width / 2. * pixel_delta_x.normalize()
            + viewport_height / 2. * up_direction;

        Self {
            pixel_width,
            pixel_height: config.pixel_height,
            center: config.center,
            pixel_delta_x,
            pixel_delta_y,
            top_left,
            focus_dist: config.focus_dist,
            aperture_angle: config.aperture_angle.to_radians(),
        }
    }

    pub fn render(&self, scene: &Scene, ray_t: Interval, samples_pr_pixel: u32) -> RgbImage {
        if samples_pr_pixel < 1 {
            panic!("Samples pr. pixel cannot be less than 1.")
        }

        let pb = ProgressBar::new(self.pixel_width as u64 * self.pixel_height as u64);
        pb.set_style(
            ProgressStyle::with_template("{elapsed_precise} {wide_bar} {pos}/{len}").unwrap(),
        );
        const BATCH_SIZE: usize = 4096;

        let mut image: RgbImage = RgbImage::new(self.pixel_width, self.pixel_height);

        let aperture = (self.aperture_angle / 2.0).tan() * self.focus_dist;
        let ap_x = self.pixel_delta_x.normalize() * aperture;
        let ap_y = self.pixel_delta_y.normalize() * aperture;

        let sample_axis = (samples_pr_pixel as f64).sqrt() as u32;
        let sample_scale = 1.0 / (sample_axis * sample_axis) as f64;
        let cell_scale = 1.0 / sample_axis as f64;
        let cell_delta_x = self.pixel_delta_x * cell_scale;
        let cell_delta_y = self.pixel_delta_y * cell_scale;

        image
            .par_enumerate_pixels_mut()
            .enumerate()
            .for_each(|(i, (x, y, pixel))| {
                let pixel_corner =
                    self.top_left + x as f64 * self.pixel_delta_x + y as f64 * self.pixel_delta_y;

                let mut color = Color::new(0.0, 0.0, 0.0);

                for i in 0..sample_axis {
                    for j in 0..sample_axis {
                        let ap_offset = Vec3::random_on_unit_disk();
                        let orig = self.center + ap_x * ap_offset.x() + ap_y * ap_offset.y();

                        let ray_dir = (pixel_corner - orig)
                            + (i as f64 + fastrand::f64()) * cell_delta_x
                            + (j as f64 + fastrand::f64()) * cell_delta_y;

                        let ray = Ray::new(orig, ray_dir);
                        color += ray_color(&ray, ray_t, scene, MAX_DEPTH);
                    }
                }

                color *= sample_scale;

                // Convert from linear space to gamma space
                let linear_to_gamma = |x: f64| x.max(0.0).sqrt();

                *pixel = color.map(linear_to_gamma).to_rgb();
                if (i + 1) % BATCH_SIZE == 0 {
                    pb.inc(BATCH_SIZE as u64);
                }
            });

        pb.finish_and_clear();

        image
    }
}

fn ray_color(ray: &Ray, ray_t: Interval, scene: &Scene, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit = scene.hit_shapes(ray, ray_t);

    if let Some(hit) = hit {
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
