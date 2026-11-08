use serde::{Deserialize, Serialize};

use crate::{
    interval::Interval,
    ray::Ray,
    shape::{Hit, Shape},
};

#[derive(Debug, Default, Serialize, Deserialize)]
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
        // let mut closest = ray_t.max();
        // let mut closest_hit = None;

        // for shape in &self.shapes {
        //     let interval = Interval::new(ray_t.min(), closest);
        //     if let Some(hit) = shape.hit(ray, interval) {
        //         closest = hit.distance;
        //         closest_hit = Some(hit);
        //     }
        // }
        // closest_hit
        
        let tree = AabbNode::new(self.shapes.iter().collect());
        tree.hit(ray, ray_t)
    }
}

//------------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct AabbNode<'a> {
    xs: Interval,
    ys: Interval,
    zs: Interval,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    shape: Option<&'a Shape>,
}

impl<'a> AabbNode<'a> {
    fn new(mut shapes: Vec<&'a Shape>) -> Self {
        assert!(!shapes.is_empty(), "AABB tree cannot be constructed from an empty vector");
        
        if shapes.len() == 1 {
            let shape = shapes[0];
            return Self {
                xs: shape.xs(),
                ys: shape.ys(),
                zs: shape.zs(),
                left: None,
                right: None,
                shape: Some(shape),
            }
        }

        let xs = find_bounding(&shapes, |a| a.xs());
        let ys = find_bounding(&shapes, |a| a.ys());
        let zs = find_bounding(&shapes, |a| a.zs());

        let [x_span, y_span, z_span] = [xs, ys, zs].map(|a| a.span());

        let get_center: fn(&&Shape) -> f64 = if x_span >= y_span && x_span >= z_span {
            |a| a.xs().center()
        } else if y_span >= z_span {
            |a| a.ys().center()
        } else {
            |a| a.zs().center()
        };

        shapes.sort_by(|a, b| {
            get_center(a).total_cmp(&get_center(b))
        });

        let right_shapes = shapes.split_off(shapes.len() / 2);

        let left_node = Self::new(shapes);
        let right_node = Self::new(right_shapes);

        Self {
            xs,
            ys,
            zs,
            left: Some(Box::new(left_node)),
            right: Some(Box::new(right_node)),
            shape: None,
        }
    }

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'a>> {
        todo!()
    }
}

fn find_bounding(shapes: &[&Shape], get_values: impl Fn(&&Shape) -> Interval) -> Interval {
    shapes
        .iter()
        .map(get_values)
        .reduce(Interval::bounding)
        .unwrap()
}
