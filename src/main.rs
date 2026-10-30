use std::time::Instant;

use anyhow::Result;
use raytracer::{
    camera::Camera,
    hittable::Hittable,
    shapes::sphere::Sphere,
    vec3::{Point, Vec3},
};

fn main() -> Result<()> {
    let shapes: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(
        Point::new(3.0, 0.0, 0.0),
        1.0,
    ))];
    let camera = Camera::new(
        16.0 / 9.0,
        1080,
        50.0,
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    let start = Instant::now();

    let image = camera.render(&shapes);

    println!("{:?}", start.elapsed());

    image.save("image.png")?;

    Ok(())
}
