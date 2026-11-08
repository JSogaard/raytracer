use std::{cmp::min_by, sync::Arc};

use crate::{
    interval::Interval,
    ray::Ray,
    shape::{Hit, Shape},
};

#[derive(Debug, Default)]
pub struct Scene {
    shapes: Vec<Arc<Shape>>,
    tree: Option<AabbNode>,
}

impl Scene {
    pub fn new(shapes: Vec<Shape>) -> Self {
        let shapes = shapes.into_iter().map(Arc::new).collect();
        Self { shapes, tree: None }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(Arc::new(shape));
        self.tree = None;
    }

    pub fn build_tree(&mut self) {
        self.tree = Some(AabbNode::new(self.shapes.clone(), MAX_DEPTH));
    }

    pub fn hit_shapes(&self, ray: &Ray, ray_t: Interval) -> Option<Hit<'_>> {
        if self.shapes.is_empty() {
            return None;
        }
        if let Some(tree) = &self.tree {
            tree.hit(ray, ray_t)
        } else {
            panic!("AABB tree has to be built before checking for hits")
        }
    }
}

//------------------------------------------------------------------------------

const EPSILON: f64 = 0.0001;
const MIN_LEAF_SIZE: usize = 4;
const MAX_DEPTH: usize = 64;

#[derive(Clone, Debug, Default)]
struct AabbNode {
    xs: Interval,
    ys: Interval,
    zs: Interval,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    shapes: Vec<Arc<Shape>>,
}

impl AabbNode {
    fn new(mut shapes: Vec<Arc<Shape>>, depth: usize) -> Self {
        assert!(
            !shapes.is_empty(),
            "AABB tree cannot be constructed from an empty vector"
        );

        let mut xs = find_bounding(&shapes, |a| a.xs());
        let mut ys = find_bounding(&shapes, |a| a.ys());
        let mut zs = find_bounding(&shapes, |a| a.zs());

        if shapes.len() <= MIN_LEAF_SIZE || depth == 0 {
            return Self {
                xs,
                ys,
                zs,
                left: None,
                right: None,
                shapes,
            };
        }

        let [x_span, y_span, z_span] = [xs, ys, zs].map(|a| a.span());

        let get_center: fn(&Shape) -> f64 = if x_span >= y_span && x_span >= z_span {
            |a| a.xs().center()
        } else if y_span >= z_span {
            |a| a.ys().center()
        } else {
            |a| a.zs().center()
        };

        shapes.sort_by(|a, b| get_center(a.as_ref()).total_cmp(&get_center(b.as_ref())));

        let right_shapes = shapes.split_off(shapes.len() / 2);

        if xs.span() < EPSILON {
            xs = xs.expand(EPSILON / 2.0);
        }
        if ys.span() < EPSILON {
            ys = ys.expand(EPSILON / 2.0);
        }
        if zs.span() < EPSILON {
            zs = zs.expand(EPSILON / 2.0);
        }

        let left_node = Self::new(shapes, depth - 1);
        let right_node = Self::new(right_shapes, depth - 1);

        Self {
            xs,
            ys,
            zs,
            left: Some(Box::new(left_node)),
            right: Some(Box::new(right_node)),
            shapes: Vec::new(),
        }
    }

    fn hit(&self, ray: &Ray, mut ray_t: Interval) -> Option<Hit<'_>> {
        if !self.shapes.is_empty() {
            return self
                .shapes
                .iter()
                .filter_map(|a| a.hit(ray, ray_t))
                .min_by(|a, b| a.distance.total_cmp(&b.distance));
        }

        let (Some(x), Some(y), Some(z)) = (
            ray.intersect_x(self.xs),
            ray.intersect_y(self.ys),
            ray.intersect_z(self.zs),
        ) else {
            return None;
        };
        if !x.overlap(&y, &z) {
            return None;
        }

        let left_hit = if let Some(left) = &self.left {
            left.hit(ray, ray_t)
        } else {
            None
        };

        if let Some(ref hit) = left_hit {
            ray_t = Interval::new(ray_t.min(), hit.distance)
        }

        let right_hit = if let Some(right) = &self.right {
            right.hit(ray, ray_t)
        } else {
            None
        };

        match (left_hit, right_hit) {
            (Some(left_hit), Some(right_hit)) => Some(min_by(left_hit, right_hit, |a, b| {
                a.distance.total_cmp(&b.distance)
            })),
            (Some(hit), None) => Some(hit),
            (None, Some(hit)) => Some(hit),
            (None, None) => None,
        }
    }
}

fn find_bounding(shapes: &[Arc<Shape>], get_values: impl Fn(&Shape) -> Interval) -> Interval {
    shapes
        .iter()
        .map(|s| get_values(s.as_ref()))
        .reduce(Interval::bounding)
        .unwrap()
}
