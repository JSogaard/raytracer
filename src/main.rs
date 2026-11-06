use std::{sync::Arc, time::Instant};

use anyhow::Result;
use raytracer::{
    camera::Camera,
    interval::Interval,
    material::{Lambertian, Metal, Refractive},
    scene::Scene,
    shape::Shape,
    vec3::{Color, Point, Vec3},
};
use raytracer::shape::Sphere;

fn main() -> Result<()> {
    let mut scene = Scene::default();

    let glass = Arc::new(Refractive::new(1.5));
    let green_diffuse = Arc::new(Lambertian::new(Color::new(0.16, 0.3, 0.12)));
    let blue_diffuse = Arc::new(Lambertian::new(Color::new(0.16, 0.08, 0.45)));
    let red_diffuse = Arc::new(Lambertian::new(Color::new(0.5, 0.04, 0.05)));
    let metal = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.2));

    let sphere = Sphere::new(Point::new(3.0, 0.0, 0.0), 1.0, glass.clone());
    scene.add_shape(Shape::Sphere(sphere));

    let sphere = Sphere::new(Point::new(1.5, 0.0, -2000.0), 1999.0, green_diffuse.clone());
    scene.add_shape(Shape::Sphere(sphere));

    let sphere = Sphere::new(Point::new(5.0, 1.2, 0.0), 1.0, blue_diffuse.clone());
    scene.add_shape(Shape::Sphere(sphere));

    let sphere = Sphere::new(Point::new(5.0, -1.2, 0.0), 1.0, metal.clone());
    scene.add_shape(Shape::Sphere(sphere));

    let sphere = Sphere::new(Point::new(5.0, 0.0, 1.2), 1.0, red_diffuse.clone());
    scene.add_shape(Shape::Sphere(sphere));

    let camera = Camera::new(
        1.0,
        1080,
        60.0,
        Point::new(0.0, 0.0, 3.0),
        Vec3::new(0.8, 0.0, -0.5),
        Vec3::new(0.0, 0.0, 1.0),
        4.0,
        0.5,
    );

    let ray_t = Interval::new(0.001, f64::INFINITY);

    let start = Instant::now();

    let image = camera.render(&scene, ray_t, 144);

    println!("{:?}", start.elapsed());

    image.save("./image.png")?;

    Ok(())
}
