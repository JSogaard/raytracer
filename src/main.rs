use std::fs;
use std::sync::Arc;
use anyhow::Result;
use raytracer::camera::CameraConfig;
use raytracer::interval::Interval;
use raytracer::shape::Sphere;
use raytracer::{
    camera::Camera,
    material::{Lambertian, Material, Metal, Refractive},
    scene::Scene,
    shape::Shape,
    vec3::{Color, Point, Vec3},
};

const LOAD: bool = true;

fn main() -> Result<()> {
    let mut scene = Scene::default();

    let ground_mat = Arc::new(Material::Lambertian(Lambertian::new(Color::new(
        0.25, 0.5, 0.25,
    ))));
    scene.add_shape(Shape::Sphere(Sphere::new(
        Point::new(0.0, 0.0, -5000.0),
        5000.0,
        ground_mat,
    )));

    for x in 0..10 {
        for y in 0..10 {
            let [x, y] = [x, y].map(|x| x as f64 * 1.5);
            let radius = fastrand::f64() * 0.15 + 0.4;
            let pos = Point::new(x + fastrand::f64() * 0.8, y + fastrand::f64() * 0.8, radius);

            let mat_num = fastrand::f64();

            let mat: Arc<Material> = if mat_num < 0.35 {
                let fuzz = fastrand::f64() * 0.1;
                Arc::new(Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), fuzz)))
            } else if mat_num < 0.75 {
                Arc::new(Material::Refractive(Refractive::new(1.5)))
            } else {
                let color = Color::random();
                Arc::new(Material::Lambertian(Lambertian::new(color)))
            };

            scene.add_shape(Shape::Sphere(Sphere::new(pos, radius, mat)));
        }
    }

    if LOAD {
        let scene_json = fs::read_to_string("scene.json")?;
        scene = serde_json::from_str(&scene_json)?;
    } else {
        let scene_json = serde_json::to_string_pretty(&scene)?;
        fs::write("scene.json", scene_json)?;
    }

    let config = CameraConfig {
        aspect_ratio: 1.0,
        pixel_height: 1080,
        vfov: 60.0,
        center: Point::new(0.0, 0.0, 3.0),
        direction: Vec3::new(1.0, 0.75, -0.6),
        up_direction: Vec3::new(0.0, 0.0, 1.0),
        focus_dist: 4.0,
        aperture_angle: 0.6,
    };

    let camera = Camera::new(config);

    let ray_t = Interval::new(0.001, f64::INFINITY);

    let start = std::time::Instant::now();

    let image = camera.render(&scene, ray_t, 10);

    println!("{:?}", start.elapsed());

    image.save("./image.png")?;

    Ok(())
}
