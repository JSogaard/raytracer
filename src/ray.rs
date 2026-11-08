use crate::{interval::Interval, vec3::{Point, Vec3}};

#[derive(Debug, Clone, Default)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Self {
            orig,
            dir: dir.normalize(),
        }
    }

    pub fn origin(&self) -> Point {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }

    pub fn intersect_x(&self, interval: Interval) -> Option<Interval> {
        axis_intersect(interval, self.orig.x(), self.dir.x())
    }

    pub fn intersect_y(&self, interval: Interval) -> Option<Interval> {
        axis_intersect(interval, self.orig.y(), self.dir.y())
    }

    pub fn intersect_z(&self, interval: Interval) -> Option<Interval> {
        axis_intersect(interval, self.orig.z(), self.dir.z())
    }
}

fn axis_intersect(interval: Interval, orig: f64, dir: f64) -> Option<Interval> {
        if dir == 0.0 {
            return interval.contains(orig).then(|| Interval::new(f64::NEG_INFINITY, f64::INFINITY));
        }
        let t0 = (interval.min() - orig) / dir;
        let t1 = (interval.max() - orig) / dir;
        Some(Interval::new(t0.min(t1), t0.max(t1)))
    }
