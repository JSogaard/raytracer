use crate::vec3::{Point, Vec3};

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
}
