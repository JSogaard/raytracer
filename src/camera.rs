use crate::vec3::{Point, Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    pixel_width: u32,
    pixel_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    center: Point,
    direction: Vec3,
    up_direction: Vec3,
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
            direction.dot(up_direction).abs() > 1e-10,
            "Camera direction and up-direction are not perpendicular"
        );

        let (pixel_delta_x, pixel_delta_y, top_left) = calc_pixel_vecs(
            pixel_height,
            center,
            pixel_width,
            viewport_height,
            viewport_width,
            direction,
            up_direction,
        );

        Self {
            pixel_width,
            pixel_height,
            viewport_width,
            viewport_height,
            center,
            direction,
            up_direction,
            pixel_delta_x,
            pixel_delta_y,
            top_left,
        }
    }
}

fn calc_pixel_vecs(
    pixel_height: u32,
    center: Vec3,
    pixel_width: u32,
    viewport_height: f64,
    viewport_width: f64,
    direction: Vec3,
    up_direction: Vec3,
) -> (Vec3, Vec3, Vec3) {
    let pixel_delta_x =
        (direction.cross(up_direction)).normalize() * viewport_width / pixel_width as f64;

    let pixel_delta_y = -up_direction * viewport_height / pixel_height as f64;

    let top_left = center + direction - viewport_width / 2. * pixel_delta_x.normalize()
        + viewport_height / 2. * up_direction
        + pixel_delta_x / 2.
        + pixel_delta_y / 2.;

    (pixel_delta_x, pixel_delta_y, top_left)
}
