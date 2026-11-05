use crate::{
    interval::Interval, ray::Ray, shape::{Hit, Shape},
};

#[derive(Debug, Default)]
pub struct Scene {
    shapes: Vec<Shape>,
}

impl Scene {
    pub fn new(shapes: Vec<Shape>) -> Self {
        Self { shapes }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn hit_shapes(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
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
