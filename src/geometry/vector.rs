extern crate num;
use std::ops::{Add, Sub};

use geometry::matrix::*;

#[derive(Debug, Clone)]
pub struct Vec3<T: num::Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: num::Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3{x: x, y: y, z: z}
    }

    fn scale(self, other: T) -> Vec3<T> {
        Vec3{x: self.x * other, y: self.y * other, z: self.z * other}
    }

    fn dot_product(self, other: Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross_product(self, other: Vec3<T>) -> Vec3<T> {
        Vec3{x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x}
    }

    fn norm(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> T {
        self.norm().sqrt()
    }

    fn normalize(self) -> Vec3<T>{
        let n = self.norm();
        let mut x = self.x;
        let mut y = self.y;
        let mut z = self.z;
        if n > num::cast::<f64, T>(0.0).unwrap() {
            let factor: T = num::cast::<f64, T>(1.0).unwrap() / n.sqrt();
            x = x * factor;
            y = y * factor;
            z = z * factor;
        }
        Vec3{x: x, y: y, z: z}
    }
}

impl<T: num::Float> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl<T: num::Float> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}





macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if $x - $y >= $d || $y - $x >= $d { panic!(); }
    }
}

fn assert_eq_float_range<T: num::Float>(n1: T, n2: T, range: T) {
    assert!((n1 - n2).abs() < range.abs());
}

#[test]
fn vec3_can_be_created() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    assert_delta!(vec1.x, 1.0, 0.001);
    assert_delta!(vec1.y, 2.0, 0.001);
    assert_delta!(vec1.z, 3.0, 0.001);
}

#[test]
fn vec3_can_be_add() {
    let vec1: Vec3<f64> = Vec3::new(4.0, 6.0, 8.0);
    let vec2: Vec3<f64> = Vec3::new(0.0, 3.0, 6.0);
    let vec3 = vec1 + vec2;
    assert_delta!(vec3.x, 4.0, 0.001);
    assert_delta!(vec3.y, 9.0, 0.001);
    assert_delta!(vec3.z, 14.0, 0.001);
}

#[test]
fn vec3_can_be_subtract() {
    let vec1: Vec3<f64> = Vec3::new(4.0, 6.0, 8.0);
    let vec2: Vec3<f64> = Vec3::new(0.0, 3.0, 6.0);
    let vec3 = vec1 - vec2;
    assert_delta!(vec3.x, 4.0, 0.001);
    assert_delta!(vec3.y, 3.0, 0.001);
    assert_delta!(vec3.z, 2.0, 0.001);
}

#[test]
fn vec3_can_scale() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    let vec2 = vec1.scale(4.0);
    assert_delta!(vec2.x, 4.0, 0.001);
    assert_delta!(vec2.y, 8.0, 0.001);
    assert_delta!(vec2.z, 12.0, 0.001);
}

#[test]
fn vec3_can_dot_product() {
    let vec1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    let vec2: Vec3<f64> = Vec3::new(3.5, 6.1, 1.9);
    let dp = vec1.dot_product(vec2);
    assert_delta!(dp, 21.4, 0.001);
}

#[test]
fn vec3_can_cross_product() {
    let vec1: Vec3<f64> = Vec3::new(2.1, 4.1, 5.1);
    let vec2: Vec3<f64> = Vec3::new(9.3, 7.5, 3.9);
    let vec3 = vec1.cross_product(vec2);
    assert_delta!(vec3.x, -22.26, 0.001);
    assert_delta!(vec3.y, 39.24, 0.001);
    assert_delta!(vec3.z, -22.38, 0.001);
}

#[test]
fn vec3_can_norm() {
    let vec1: Vec3<f64> = Vec3::new(1.1, 2.2, 3.3);
    let norm = vec1.norm();
    assert_delta!(norm, 16.94, 0.001);
}

#[test]
fn vec3_can_length() {
    let vec1: Vec3<f64> = Vec3::new(1.1, 2.2, 3.3);
    let length = vec1.length();
    assert_delta!(length, 4.1158, 0.001);
}

#[test]
fn vec3_can_normalize() {
    let vec1: Vec3<f64> = Vec3::new(3.0, 2.0, -1.0);
    let vec2 = vec1.normalize();
    assert_delta!(vec2.x, 0.8018, 0.001);
    assert_delta!(vec2.y, 0.5345, 0.001);
    assert_delta!(vec2.z, -0.2673, 0.001);
}