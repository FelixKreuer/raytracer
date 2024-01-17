// math/Vector3.rs

use std::fmt;
use std::ops::{Add, Mul, Sub};
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub type Point3 = Vector3;

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f32) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul for Vector3 {
    type Output = f32;
    fn mul(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = if self.x == -0.0 { 0.0 } else { self.x };
        let y = if self.y == -0.0 { 0.0 } else { self.y };
        let z = if self.z == -0.0 { 0.0 } else { self.z };
        write!(f, "Vector3({}, {}, {})", x, y, z)
    }
}
