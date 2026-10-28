use image::Rgb;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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

    pub const fn x(&self) -> f64 {
        self.c[0]
    }

    pub const fn y(&self) -> f64 {
        self.c[1]
    }

    pub const fn z(&self) -> f64 {
        self.c[2]
    }

    pub const fn coords(&self) -> [f64; 3] {
        [self.x(), self.y(), self.z()]
    }

    pub fn length_squared(&self) -> f64 {
        self.x().powf(2.) + self.y().powf(2.) + self.z().powf(2.)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        
        let x = self.x() / length;
        let y = self.y() / length;
        let z = self.z() / length;

        Self::new(x, y, z)
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

    pub fn to_rgb(&self) -> Rgb<u8> {
        let x = (self.x().clamp(0., 1.0) * 255.999) as u8;
        let y = (self.y().clamp(0., 1.0) * 255.999) as u8;
        let z = (self.z().clamp(0., 1.0) * 255.999) as u8;

        Rgb([x, y, z])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Element-wise multiplication
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Element-wise division
impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

/// Scalar multiplication
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x() * self, rhs.y() * self, rhs.z() * self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

/// Scalar division
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = write!(f, "({} {} {})", self.x(), self.y(), self.z());
        result
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x(), self.y(), self.z())
    }
}

pub type Point = Vec3;
pub type Color = Vec3;
