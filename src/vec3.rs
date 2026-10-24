use std::ops::{Add, Div, Mul, Sub};

// Vector constants
pub const UNIT_X: Vec3 = Vec3::new(1., 0., 0.);
pub const UNIT_Y: Vec3 = Vec3::new(0., 1., 0.);
pub const UNIT_Z: Vec3 = Vec3::new(0., 0., 1.);
pub const ZERO_VEC: Vec3 = Vec3::new(0., 0., 0.);

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Vec3 {
    c: [f64; 3],
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { c: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.c[0]
    }

    pub fn y(&self) -> f64 {
        self.c[1]
    }

    pub fn z(&self) -> f64 {
        self.c[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.x().powf(2.) + self.y().powf(2.) + self.z().powf(2.)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Self) -> Self {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = -(self.x() * other.z() - self.z() * other.x());
        let z = self.x() * other.y() - self.y() * other.x();

        Self::new(x, y, z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

/// Element-wise multiplication
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

/// Element-wise division
impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

/// Scalar multiplication
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

/// Scalar division
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
