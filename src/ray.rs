use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub const fn new(orig: Point, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub const fn origin(&self) -> Point {
        self.orig
    }

    pub const fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at(&self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
