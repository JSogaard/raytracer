use std::{sync::Arc, time::Instant};

use anyhow::Result;
use raytracer::{
    camera::Camera,
    interval::Interval,
    lambertian::Lambertian,
    scene::Scene,
    shapes::sphere::Sphere,
    vec3::{Color, Point, Vec3},
};

fn main() -> Result<()> {
    let mut scene = Scene::default();

    let lambertian = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let circle = Sphere::new(Point::new(3.0, 0.0, 0.0), 1.0, lambertian.clone());
    scene.add_shape(Box::new(circle));

    let circle = Sphere::new(Point::new(1.5, 0.0, -2000.0), 1999.0, lambertian.clone());
    scene.add_shape(Box::new(circle));

    let circle = Sphere::new(Point::new(3.0, 2000.0, 0.0), -1999.0, lambertian.clone());
    scene.add_shape(Box::new(circle));

    let camera = Camera::new(
        1.0,
        800,
        50.0,
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        15,
    );

    let ray_t = Interval::new(0.001, f64::INFINITY);

    let start = Instant::now();

    let image = camera.render(&scene, ray_t);

    println!("{:?}", start.elapsed());

    image.save("image.png")?;

    Ok(())
}
