use std::time::Instant;

use anyhow::Result;
use raytracer::{
    camera::Camera, hittable::Hittable, interval::Interval, shapes::sphere::Sphere, vec3::{Point, Vec3},
};

fn main() -> Result<()> {
    let shapes: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(3.0, 0.0, 0.0), 1.0)),
        Box::new(Sphere::new(Point::new(1.5, 0.0, -2000.0), -1998.0)),
    ];
    let camera = Camera::new(
        16.0 / 9.0,
        1080,
        50.0,
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    let ray_t = Interval::new(0.0, f64::INFINITY);

    let start = Instant::now();

    let image = camera.render(&shapes, ray_t);

    println!("{:?}", start.elapsed());

    image.save("image.png")?;

    Ok(())
}
