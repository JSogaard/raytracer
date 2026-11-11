use crate::{
    interval::Interval, ray::Ray, shape::{Hit, Hittable},
};
use std::{cmp::min_by, sync::Arc};

#[derive(Debug, Default)]
pub struct Scene {
    shapes: Vec<Arc<dyn Hittable>>,
    tree: Option<AabbNode>,
}

impl Scene {
    pub fn new(shapes: Vec<Arc<dyn Hittable>>) -> Self {
        let shapes = shapes;
        Self { shapes, tree: None }
    }

    pub fn add_shape(&mut self, shape: Arc<dyn Hittable>) {
        self.shapes.push(shape);
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
            None
        }
    }
}

//------------------------------------------------------------------------------

const EPSILON: f64 = 0.0001;
const MIN_LEAF_SIZE: usize = 4;
const MAX_DEPTH: usize = 64;
const SAH_BINS: usize = 16;

#[derive(Clone, Debug, Default)]
struct AabbNode {
    xs: Interval,
    ys: Interval,
    zs: Interval,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    shapes: Vec<Arc<dyn Hittable>>,
}

impl AabbNode {
    fn new(shapes: Vec<Arc<dyn Hittable>>, depth: usize) -> Self {
        assert!(
            !shapes.is_empty(),
            "AABB tree cannot be constructed from an empty vector"
        );

        let mut xs = find_bounding(&shapes, |s| s.xs()).unwrap();
        let mut ys = find_bounding(&shapes, |s| s.ys()).unwrap();
        let mut zs = find_bounding(&shapes, |s| s.zs()).unwrap();

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

        let [x_span, y_span, z_span] = [&xs, &ys, &zs].map(|a| a.span());

        let axis = if x_span >= y_span && x_span >= z_span {
            |s: &dyn Hittable| s.xs()
        } else if y_span >= z_span {
            |s: &dyn Hittable| s.ys()
        } else {
            |s: &dyn Hittable| s.zs()
        };

        let (left_shapes, right_shapes) = split_box(shapes, axis);

        if xs.span() < EPSILON {
            xs = xs.expand(EPSILON / 2.0);
        }
        if ys.span() < EPSILON {
            ys = ys.expand(EPSILON / 2.0);
        }
        if zs.span() < EPSILON {
            zs = zs.expand(EPSILON / 2.0);
        }

        let left_node = Self::new(left_shapes, depth - 1);
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

fn find_bounding<'a, I>(shapes: I, axis: fn(&dyn Hittable) -> Interval) -> Option<Interval>
where
    I: IntoIterator<Item = &'a Arc<dyn Hittable>>,
{
    shapes
        .into_iter()
        .map(|s| axis(s.as_ref()))
        .reduce(Interval::bounding)
}

type SplitShapes = (Vec<Arc<dyn Hittable>>, Vec<Arc<dyn Hittable>>);

fn split_box(mut shapes: Vec<Arc<dyn Hittable>>, axis: fn(&dyn Hittable) -> Interval) -> SplitShapes {
    let min_center = shapes
        .iter()
        .map(|shape| axis(shape.as_ref()).center())
        .min_by(f64::total_cmp)
        .unwrap();
    let max_center = shapes
        .iter()
        .map(|shape| axis(shape.as_ref()).center())
        .max_by(f64::total_cmp)
        .unwrap();
    let total_span = max_center - min_center;

    if total_span == 0.0 {
        let right_shapes = shapes.split_off(shapes.len() / 2);
        return (shapes, right_shapes);
    }

    let bin_number = |shape: &Arc<dyn Hittable>| {
        (SAH_BINS as f64 * (axis(shape.as_ref()).center() - min_center) / total_span)
            .clamp(0.0, SAH_BINS as f64 - 1.0) as usize
    };

    shapes.sort_unstable_by_key(|s| bin_number(s));

    let mut bin_counts = [0_usize; SAH_BINS];
    let mut bin_bounds: [Option<[Interval; 3]>; SAH_BINS] = [None; SAH_BINS];

    for shape in &shapes {
        let bin = bin_number(shape);
        bin_counts[bin] += 1;

        let shape_bounds = [shape.xs(), shape.ys(), shape.zs()];

        bin_bounds[bin] = Some(match bin_bounds[bin] {
            Some(bounds) => Interval::bounding_3d(&bounds, &shape_bounds),
            None => shape_bounds,
        })
    }

    let mut split_costs: Vec<f64> = Vec::new();

    for split in 1..SAH_BINS {
        let left_bounds = find_split_bounds(&bin_bounds, 0, split);
        let left_surface = if let Some(bounds) = left_bounds {
            surface_area(bounds)
        } else {
            0.0
        };
        let left_count = bin_counts[0..split].iter().sum::<usize>();

        let right_bounds = find_split_bounds(&bin_bounds, split, SAH_BINS);
        let right_surface = if let Some(bounds) = right_bounds {
            surface_area(bounds)
        } else {
            0.0
        };
        let right_count = bin_counts[split..SAH_BINS].iter().sum::<usize>();

        if left_count == 0 || right_count == 0 {
            split_costs.push(f64::INFINITY);
            continue;
        }

        let cost = left_surface * left_count as f64 + right_surface * right_count as f64;
        split_costs.push(cost);
    }

    let split_bin = split_costs
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(b.1))
        .unwrap()
        .0
        + 1;

    let left_count = bin_counts[0..split_bin].iter().sum();

    let right_shapes = shapes.split_off(left_count);

    (shapes, right_shapes)
}

fn find_split_bounds(
    bin_bounds: &[Option<[Interval; 3]>],
    start: usize,
    end: usize,
) -> Option<[Interval; 3]> {
    bin_bounds[start..end]
        .iter()
        .fold(None, |acc, &elem| match (acc, elem) {
            (Some(acc), Some(elem)) => Some(Interval::bounding_3d(&acc, &elem)),
            (Some(acc), None) => Some(acc),
            (None, Some(elem)) => Some(elem),
            (None, None) => None,
        })
}

fn surface_area(bounds: [Interval; 3]) -> f64 {
    let dx = bounds[0].span();
    let dy = bounds[1].span();
    let dz = bounds[2].span();

    2.0 * (dx * dy + dy * dz + dx * dz)
}
