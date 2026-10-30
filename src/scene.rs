use crate::{
    hittable::{Hit, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Debug, Default)]
pub struct Scene {
    shapes: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(shapes: Vec<Box<dyn Hittable>>) -> Self {
        Self { shapes }
    }

    pub fn push_shape(&mut self, shape: Box<dyn Hittable>) {
        self.shapes.push(shape);
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        // self.shapes.iter()
        //     .fold((ray_t.max(), None), |(closest, closest_hit), next| {
        //         match next.hit(ray, Interval::new(ray_t.min(), closest)) {
        //             Some(hit) => (hit.distance, Some(hit)),
        //             None => (closest, closest_hit)
        //         }
        // }).1

        let mut closest = ray_t.max();
        let mut closest_hit = None;

        for shape in &self.shapes {
            let interval = Interval::new(ray_t.min(), closest);
            if let Some(hit) = shape.hit(ray, interval) {
                closest = hit.distance;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
