extern crate num;
use std::ops::{Mul};
use std::mem;
use std::ops::{Index, IndexMut};

use geometry::vector::*;

#[derive(Debug, Clone)]
pub struct Matrix44<T: num::Float> {
    m11: T, m12: T, m13: T, m14: T,
    m21: T, m22: T, m23: T, m24: T,
    m31: T, m32: T, m33: T, m34: T,
    m41: T, m42: T, m43: T, m44: T,
}

impl<T: num::Float> Matrix44<T> {
    pub fn new(m11: T, m12: T, m13: T, m14: T, m21: T, m22: T, m23: T, m24: T, m31: T, m32: T, m33: T, m34: T, m41: T, m42: T, m43: T, m44: T) -> Matrix44<T> {
        Matrix44{m11 : m11, m12 : m12, m13 : m13, m14 : m14,
                 m21 : m21, m22 : m22, m23 : m23, m24 : m24,
                 m31 : m31, m32 : m32, m33 : m33, m34 : m34,
                 m41 : m41, m42 : m42, m43 : m43, m44 : m44}
    }

    fn new_single(m: T) -> Matrix44<T> {
        Matrix44{m11 : m, m12 : m, m13 : m, m14 : m,
                 m21 : m, m22 : m, m23 : m, m24 : m,
                 m31 : m, m32 : m, m33 : m, m34 : m,
                 m41 : m, m42 : m, m43 : m, m44 : m}
    }

    fn new_identity() -> Matrix44<T> {
        let zero: T = num::cast::<f64, T>(0.0).unwrap();
        let one: T = num::cast::<f64, T>(1.0).unwrap();

        Matrix44{m11 : one, m12 : zero, m13 : zero, m14 : zero,
                 m21 : zero, m22 : one, m23 : zero, m24 : zero,
                 m31 : zero, m32 : zero, m33 : one, m34 : zero,
                 m41 : zero, m42 : zero, m43 : zero, m44 : one}
    }

    fn transpose(self) -> Matrix44<T> {
        let value: T = num::cast::<f64, T>(0.0).unwrap();
        let mut matrix : Matrix44<T> = Matrix44::new_single(value);
        
        for i in 0..4 {
            for j in 0..4 {
                matrix[(i, j)] = self[(j, i)]
            }
        }

        matrix
    }

    pub fn inverse(self) -> Matrix44<T> {
        let value: T = num::cast::<f64, T>(0.0).unwrap();
        let one: T = num::cast::<f64, T>(1.0).unwrap();
        let mut matrix : Matrix44<T> = Matrix44::new_single(value);
        
        let det = self[(0, 3)]*self[(1, 2)]*self[(2, 1)]*self[(3, 0)] - self[(0, 2)]*self[(1, 3)]*self[(2, 1)]*self[(3, 0)] - self[(0, 3)]*self[(1, 1)]*self[(2, 2)]*self[(3, 0)] + self[(0, 1)]*self[(1, 3)]*self[(2, 2)]*self[(3, 0)] +
                  self[(0, 2)]*self[(1, 1)]*self[(2, 3)]*self[(3, 0)] - self[(0, 1)]*self[(1, 2)]*self[(2, 3)]*self[(3, 0)] - self[(0, 3)]*self[(1, 2)]*self[(2, 0)]*self[(3, 1)] + self[(0, 2)]*self[(1, 3)]*self[(2, 0)]*self[(3, 1)] +
                  self[(0, 3)]*self[(1, 0)]*self[(2, 2)]*self[(3, 1)] - self[(0, 0)]*self[(1, 3)]*self[(2, 2)]*self[(3, 1)] - self[(0, 2)]*self[(1, 0)]*self[(2, 3)]*self[(3, 1)] + self[(0, 0)]*self[(1, 2)]*self[(2, 3)]*self[(3, 1)] +
                  self[(0, 3)]*self[(1, 1)]*self[(2, 0)]*self[(3, 2)] - self[(0, 1)]*self[(1, 3)]*self[(2, 0)]*self[(3, 2)] - self[(0, 3)]*self[(1, 0)]*self[(2, 1)]*self[(3, 2)] + self[(0, 0)]*self[(1, 3)]*self[(2, 1)]*self[(3, 2)] +
                  self[(0, 1)]*self[(1, 0)]*self[(2, 3)]*self[(3, 2)] - self[(0, 0)]*self[(1, 1)]*self[(2, 3)]*self[(3, 2)] - self[(0, 2)]*self[(1, 1)]*self[(2, 0)]*self[(3, 3)] + self[(0, 1)]*self[(1, 2)]*self[(2, 0)]*self[(3, 3)] +
                  self[(0, 2)]*self[(1, 0)]*self[(2, 1)]*self[(3, 3)] - self[(0, 0)]*self[(1, 2)]*self[(2, 1)]*self[(3, 3)] - self[(0, 1)]*self[(1, 0)]*self[(2, 2)]*self[(3, 3)] + self[(0, 0)]*self[(1, 1)]*self[(2, 2)]*self[(3, 3)];

        matrix[(0, 0)] = self[(1, 2)]*self[(2, 3)]*self[(3, 1)] - self[(1, 3)]*self[(2, 2)]*self[(3, 1)] + self[(1, 3)]*self[(2, 1)]*self[(3, 2)] - self[(1, 1)]*self[(2, 3)]*self[(3, 2)] - self[(1, 2)]*self[(2, 1)]*self[(3, 3)] + self[(1, 1)]*self[(2, 2)]*self[(3, 3)];
        matrix[(0, 1)] = self[(0, 3)]*self[(2, 2)]*self[(3, 1)] - self[(0, 2)]*self[(2, 3)]*self[(3, 1)] - self[(0, 3)]*self[(2, 1)]*self[(3, 2)] + self[(0, 1)]*self[(2, 3)]*self[(3, 2)] + self[(0, 2)]*self[(2, 1)]*self[(3, 3)] - self[(0, 1)]*self[(2, 2)]*self[(3, 3)];
        matrix[(0, 2)] = self[(0, 2)]*self[(1, 3)]*self[(3, 1)] - self[(0, 3)]*self[(1, 2)]*self[(3, 1)] + self[(0, 3)]*self[(1, 1)]*self[(3, 2)] - self[(0, 1)]*self[(1, 3)]*self[(3, 2)] - self[(0, 2)]*self[(1, 1)]*self[(3, 3)] + self[(0, 1)]*self[(1, 2)]*self[(3, 3)];
        matrix[(0, 3)] = self[(0, 3)]*self[(1, 2)]*self[(2, 1)] - self[(0, 2)]*self[(1, 3)]*self[(2, 1)] - self[(0, 3)]*self[(1, 1)]*self[(2, 2)] + self[(0, 1)]*self[(1, 3)]*self[(2, 2)] + self[(0, 2)]*self[(1, 1)]*self[(2, 3)] - self[(0, 1)]*self[(1, 2)]*self[(2, 3)];
        matrix[(1, 0)] = self[(1, 3)]*self[(2, 2)]*self[(3, 0)] - self[(1, 2)]*self[(2, 3)]*self[(3, 0)] - self[(1, 3)]*self[(2, 0)]*self[(3, 2)] + self[(1, 0)]*self[(2, 3)]*self[(3, 2)] + self[(1, 2)]*self[(2, 0)]*self[(3, 3)] - self[(1, 0)]*self[(2, 2)]*self[(3, 3)];
        matrix[(1, 1)] = self[(0, 2)]*self[(2, 3)]*self[(3, 0)] - self[(0, 3)]*self[(2, 2)]*self[(3, 0)] + self[(0, 3)]*self[(2, 0)]*self[(3, 2)] - self[(0, 0)]*self[(2, 3)]*self[(3, 2)] - self[(0, 2)]*self[(2, 0)]*self[(3, 3)] + self[(0, 0)]*self[(2, 2)]*self[(3, 3)];
        matrix[(1, 2)] = self[(0, 3)]*self[(1, 2)]*self[(3, 0)] - self[(0, 2)]*self[(1, 3)]*self[(3, 0)] - self[(0, 3)]*self[(1, 0)]*self[(3, 2)] + self[(0, 0)]*self[(1, 3)]*self[(3, 2)] + self[(0, 2)]*self[(1, 0)]*self[(3, 3)] - self[(0, 0)]*self[(1, 2)]*self[(3, 3)];
        matrix[(1, 3)] = self[(0, 2)]*self[(1, 3)]*self[(2, 0)] - self[(0, 3)]*self[(1, 2)]*self[(2, 0)] + self[(0, 3)]*self[(1, 0)]*self[(2, 2)] - self[(0, 0)]*self[(1, 3)]*self[(2, 2)] - self[(0, 2)]*self[(1, 0)]*self[(2, 3)] + self[(0, 0)]*self[(1, 2)]*self[(2, 3)];
        matrix[(2, 0)] = self[(1, 1)]*self[(2, 3)]*self[(3, 0)] - self[(1, 3)]*self[(2, 1)]*self[(3, 0)] + self[(1, 3)]*self[(2, 0)]*self[(3, 1)] - self[(1, 0)]*self[(2, 3)]*self[(3, 1)] - self[(1, 1)]*self[(2, 0)]*self[(3, 3)] + self[(1, 0)]*self[(2, 1)]*self[(3, 3)];
        matrix[(2, 1)] = self[(0, 3)]*self[(2, 1)]*self[(3, 0)] - self[(0, 1)]*self[(2, 3)]*self[(3, 0)] - self[(0, 3)]*self[(2, 0)]*self[(3, 1)] + self[(0, 0)]*self[(2, 3)]*self[(3, 1)] + self[(0, 1)]*self[(2, 0)]*self[(3, 3)] - self[(0, 0)]*self[(2, 1)]*self[(3, 3)];
        matrix[(2, 2)] = self[(0, 1)]*self[(1, 3)]*self[(3, 0)] - self[(0, 3)]*self[(1, 1)]*self[(3, 0)] + self[(0, 3)]*self[(1, 0)]*self[(3, 1)] - self[(0, 0)]*self[(1, 3)]*self[(3, 1)] - self[(0, 1)]*self[(1, 0)]*self[(3, 3)] + self[(0, 0)]*self[(1, 1)]*self[(3, 3)];
        matrix[(2, 3)] = self[(0, 3)]*self[(1, 1)]*self[(2, 0)] - self[(0, 1)]*self[(1, 3)]*self[(2, 0)] - self[(0, 3)]*self[(1, 0)]*self[(2, 1)] + self[(0, 0)]*self[(1, 3)]*self[(2, 1)] + self[(0, 1)]*self[(1, 0)]*self[(2, 3)] - self[(0, 0)]*self[(1, 1)]*self[(2, 3)];
        matrix[(3, 0)] = self[(1, 2)]*self[(2, 1)]*self[(3, 0)] - self[(1, 1)]*self[(2, 2)]*self[(3, 0)] - self[(1, 2)]*self[(2, 0)]*self[(3, 1)] + self[(1, 0)]*self[(2, 2)]*self[(3, 1)] + self[(1, 1)]*self[(2, 0)]*self[(3, 2)] - self[(1, 0)]*self[(2, 1)]*self[(3, 2)];
        matrix[(3, 1)] = self[(0, 1)]*self[(2, 2)]*self[(3, 0)] - self[(0, 2)]*self[(2, 1)]*self[(3, 0)] + self[(0, 2)]*self[(2, 0)]*self[(3, 1)] - self[(0, 0)]*self[(2, 2)]*self[(3, 1)] - self[(0, 1)]*self[(2, 0)]*self[(3, 2)] + self[(0, 0)]*self[(2, 1)]*self[(3, 2)];
        matrix[(3, 2)] = self[(0, 2)]*self[(1, 1)]*self[(3, 0)] - self[(0, 1)]*self[(1, 2)]*self[(3, 0)] - self[(0, 2)]*self[(1, 0)]*self[(3, 1)] + self[(0, 0)]*self[(1, 2)]*self[(3, 1)] + self[(0, 1)]*self[(1, 0)]*self[(3, 2)] - self[(0, 0)]*self[(1, 1)]*self[(3, 2)];
        matrix[(3, 3)] = self[(0, 1)]*self[(1, 2)]*self[(2, 0)] - self[(0, 2)]*self[(1, 1)]*self[(2, 0)] + self[(0, 2)]*self[(1, 0)]*self[(2, 1)] - self[(0, 0)]*self[(1, 2)]*self[(2, 1)] - self[(0, 1)]*self[(1, 0)]*self[(2, 2)] + self[(0, 0)]*self[(1, 1)]*self[(2, 2)];

        let scaled_matrix = matrix.scale(one/det);

        scaled_matrix
    }

    pub fn inverse2(self) -> Matrix44<T> {
        let zero: T = num::cast::<f64, T>(0.0).unwrap();
        let mut inverse : Matrix44<T> = self;
        let mut identity : Matrix44<T> = Matrix44::new_identity();

        /* Gauss Jordan elimination */
        for k in 0..4 {


            /*** If row k has a 0 at column k, then we need to swap it
                 with a different row so that we have a non zero value
                 at [(k, k)]. */

            let mut row_with_nonzero_kk = k;

            /* Find a row that has a value in column k. */
            while row_with_nonzero_kk < 4 {
                if inverse[(row_with_nonzero_kk, k)] != zero {
                    break
                }
                row_with_nonzero_kk = row_with_nonzero_kk + 1
            }

            /* If there were no rows with a value in column k, we can't invert this matrix. */
            if row_with_nonzero_kk == 4 {
                panic!();
            }

            /* If we need to, swap the rows k and row_with_nonzero_kk to get the row with a value in column k
               into row k. */
            if(row_with_nonzero_kk != k) {
                for j in 0..4 {
                    let mut temp = zero;

                    temp = inverse[(row_with_nonzero_kk, j)];
                    inverse[(row_with_nonzero_kk, j)] = inverse[(k, j)];
                    inverse[(k, j)] = temp;

                    temp = identity[(row_with_nonzero_kk, j)];
                    identity[(row_with_nonzero_kk, j)] = identity[(k, j)];
                    identity[(k, j)] = temp;
                }
            }



            /*** Scale row k so that position [(k, k)] is equal to 1.0 */

            let pivot = inverse[(k, k)];

            /* Use the pivot value to scale the value at [k][k] to 1, by scaling 
               the entire row. */
            /* Todo: Do we want to skip the left columns of row k, because they should
               be 0? */
            for col in 0..4 {
                inverse[(k, col)] = inverse[(k, col)] / pivot;
                identity[(k, col)] = identity[(k, col)] / pivot;
            }

            /* Now the row has a 1 at position [k][k]. We need to make all of the
               other rows in column k have a value of 0 by adding/subtracting
               multiples of row k to/from them. */
            for row_to_0_col_k in 0..4 {
                if row_to_0_col_k != k {
                    let normalizer = inverse[(row_to_0_col_k, k)];

                    for col in 0..4 {
                        inverse[(row_to_0_col_k, col)] = inverse[(row_to_0_col_k, col)] - inverse[(k, col)] * normalizer;
                        identity[(row_to_0_col_k, col)] = identity[(row_to_0_col_k, col)] - identity[(k, col)] * normalizer;
                    }
                }
            }
        }

        identity
    }

    fn scale(self, scale: T) -> Matrix44<T> {
        let value: T = num::cast::<f64, T>(0.0).unwrap();
        let mut matrix : Matrix44<T> = Matrix44::new_single(value);

        for i in 0..4 {
            for j in 0..4 {
                matrix[(i, j)] = self[(i, j)] * scale
            }
        }

        matrix
    }

    pub fn multVecMatrix(&self, src: &Vec3<T>) -> Vec3<T> {
        let a: T = src.x * self[(0, 0)] + src.y * self[(1, 0)] + src.z * self[(2, 0)] + self[(3, 0)];
        let b: T = src.x * self[(0, 1)] + src.y * self[(1, 1)] + src.z * self[(2, 1)] + self[(3, 1)];
        let c: T = src.x * self[(0, 2)] + src.y * self[(1, 2)] + src.z * self[(2, 2)] + self[(3, 2)];
        let w: T = src.x * self[(0, 3)] + src.y * self[(1, 3)] + src.z * self[(2, 3)] + self[(3, 3)];

        Vec3::new(a / w, b / w, c / w)
    }

    fn multDirMatrix(&self, src: &Vec3<T>) -> Vec3<T> {
        let a: T = src.x * self[(0, 0)] + src.y * self[(1, 0)] + src.z * self[(2, 0)];
        let b: T = src.x * self[(0, 1)] + src.y * self[(1, 1)] + src.z * self[(2, 1)];
        let c: T = src.x * self[(0, 2)] + src.y * self[(1, 2)] + src.z * self[(2, 2)];

        Vec3::new(a, b, c)
    }
}

impl<T: num::Float> Index<(usize, usize)> for Matrix44<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &T {
        unsafe {
            &mem::transmute::<&Matrix44<T>, & [T; 4 * 4]>(self)[4 * i + j]
        }
    }
}

impl<T: num::Float> IndexMut<(usize, usize)> for Matrix44<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        unsafe {
            &mut mem::transmute::<&mut Matrix44<T>, &mut [T; 4 * 4]>(self)[4 * i + j]
        }
    }
}

impl<T: num::Float> Mul for Matrix44<T> {
    type Output = Matrix44<T>;

    fn mul(self, other: Matrix44<T>) -> Matrix44<T> {
        let value: T = num::cast::<f64, T>(0.0).unwrap();
        let mut matrix : Matrix44<T> = Matrix44::new_single(value);
        
        for i in 0..4 {
            for j in 0..4 {
                matrix[(i, j)] = self[(i, 0)] * other[(0, j)] + self[(i, 1)] * other[(1, j)] + self[(i, 2)] * other[(2, j)] + self[(i, 3)] * other[(3, j)];
            }
        }

        matrix
    }
}





macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if $x - $y >= $d || $y - $x >= $d { panic!("x: {:?}, y: {:?}, d: {:?}", $x, $y, $d); }
    }
}

fn assert_eq_float_range<T: num::Float>(n1: T, n2: T, range: T) {
    assert!((n1 - n2).abs() < range.abs());
}

#[test]
fn matrix44_can_be_created() {
    let mat1: Matrix44<f64> = Matrix44::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    assert_delta!(mat1.m11, 1.0, 0.001);
    assert_delta!(mat1.m12, 2.0, 0.001);
    assert_delta!(mat1.m13, 3.0, 0.001);
    assert_delta!(mat1.m14, 4.0, 0.001);
    assert_delta!(mat1.m21, 5.0, 0.001);
    assert_delta!(mat1.m22, 6.0, 0.001);
    assert_delta!(mat1.m23, 7.0, 0.001);
    assert_delta!(mat1.m24, 8.0, 0.001);
    assert_delta!(mat1.m31, 9.0, 0.001);
    assert_delta!(mat1.m32, 10.0, 0.001);
    assert_delta!(mat1.m33, 11.0, 0.001);
    assert_delta!(mat1.m34, 12.0, 0.001);
    assert_delta!(mat1.m41, 13.0, 0.001);
    assert_delta!(mat1.m42, 14.0, 0.001);
    assert_delta!(mat1.m43, 15.0, 0.001);
    assert_delta!(mat1.m44, 16.0, 0.001);
}

#[test]
fn matrix44_can_be_multiplied() {
    let mat1: Matrix44<f64> = Matrix44::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    let mat2: Matrix44<f64> = Matrix44::new(1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.10, 11.11, 12.12, 13.13, 14.14, 15.15, 16.16);
    let mat3 = mat1 * mat2;
    assert_delta!(mat3.m11, 94.32, 0.001);
    assert_delta!(mat3.m12, 102.26, 0.001);
    assert_delta!(mat3.m13, 112.63, 0.001);
    assert_delta!(mat3.m14, 123.0, 0.001);
    assert_delta!(mat3.m21, 212.84, 0.001);
    assert_delta!(mat3.m22, 234.42, 0.001);
    assert_delta!(mat3.m23, 261.67, 0.001);
    assert_delta!(mat3.m24, 288.92, 0.001);
    assert_delta!(mat3.m31, 331.36, 0.001);
    assert_delta!(mat3.m32, 366.58, 0.001);
    assert_delta!(mat3.m33, 410.71, 0.001);
    assert_delta!(mat3.m34, 454.84, 0.001);
    assert_delta!(mat3.m41, 449.88, 0.001);
    assert_delta!(mat3.m42, 498.74, 0.001);
    assert_delta!(mat3.m43, 559.75, 0.001);
    assert_delta!(mat3.m44, 620.76, 0.001);
}

#[test]
fn matrix44_can_be_transposed() {
    let mat1: Matrix44<f64> = Matrix44::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    let mat2 = mat1.transpose();
    assert_delta!(mat2.m11, 1.0, 0.001);
    assert_delta!(mat2.m12, 5.0, 0.001);
    assert_delta!(mat2.m13, 9.0, 0.001);
    assert_delta!(mat2.m14, 13.0, 0.001);
    assert_delta!(mat2.m21, 2.0, 0.001);
    assert_delta!(mat2.m22, 6.0, 0.001);
    assert_delta!(mat2.m23, 10.0, 0.001);
    assert_delta!(mat2.m24, 14.0, 0.001);
    assert_delta!(mat2.m31, 3.0, 0.001);
    assert_delta!(mat2.m32, 7.0, 0.001);
    assert_delta!(mat2.m33, 11.0, 0.001);
    assert_delta!(mat2.m34, 15.0, 0.001);
    assert_delta!(mat2.m41, 4.0, 0.001);
    assert_delta!(mat2.m42, 8.0, 0.001);
    assert_delta!(mat2.m43, 12.0, 0.001);
    assert_delta!(mat2.m44, 16.0, 0.001);
}

#[test]
fn matrix44_can_be_scaled() {
    let mat1: Matrix44<f64> = Matrix44::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
    let mat2 = mat1.scale(0.25);
    assert_delta!(mat2.m11, 0.25, 0.001);
    assert_delta!(mat2.m12, 0.5, 0.001);
    assert_delta!(mat2.m13, 0.75, 0.001);
    assert_delta!(mat2.m14, 1.0, 0.001);
    assert_delta!(mat2.m21, 1.25, 0.001);
    assert_delta!(mat2.m22, 1.5, 0.001);
    assert_delta!(mat2.m23, 1.75, 0.001);
    assert_delta!(mat2.m24, 2.0, 0.001);
    assert_delta!(mat2.m31, 2.25, 0.001);
    assert_delta!(mat2.m32, 2.5, 0.001);
    assert_delta!(mat2.m33, 2.75, 0.001);
    assert_delta!(mat2.m34, 3.0, 0.001);
    assert_delta!(mat2.m41, 3.25, 0.001);
    assert_delta!(mat2.m42, 3.5, 0.001);
    assert_delta!(mat2.m43, 3.75, 0.001);
    assert_delta!(mat2.m44, 4.0, 0.001);
}

#[test]
fn matrix44_can_be_inverted() {
    let mat1: Matrix44<f64> = Matrix44::new(0.707107, 0.0, -0.707107, 0.0, -0.331295, 0.883452, -0.331295, 0.0, 0.624695, 0.468521, 0.624695, 0.0, 4.000574, 3.00043, 4.000574, 1.0);
    let mat2 = mat1.inverse();
    assert_delta!(mat2.m11, 0.707107, 0.001);
    assert_delta!(mat2.m12, -0.331295, 0.001);
    assert_delta!(mat2.m13, 0.624695, 0.001);
    assert_delta!(mat2.m14, 0.0, 0.001);
    assert_delta!(mat2.m21, 0.0, 0.001);
    assert_delta!(mat2.m22, 0.883452, 0.001);
    assert_delta!(mat2.m23, 0.468521, 0.001);
    assert_delta!(mat2.m24, 0.0, 0.001);
    assert_delta!(mat2.m31, -0.707107, 0.001);
    assert_delta!(mat2.m32, -0.331295, 0.001);
    assert_delta!(mat2.m33, 0.624695, 0.001);
    assert_delta!(mat2.m34, 0.0, 0.001);
    assert_delta!(mat2.m41, 0.0, 0.001);
    assert_delta!(mat2.m42, 0.0, 0.001);
    assert_delta!(mat2.m43, -6.404043, 0.001);
    assert_delta!(mat2.m44, 1.0, 0.001);
}

#[test]
fn matrix44_can_be_inverted2() {
    let mat1: Matrix44<f64> = Matrix44::new(0.707107, 0.0, -0.707107, 0.0, -0.331295, 0.883452, -0.331295, 0.0, 0.624695, 0.468521, 0.624695, 0.0, 4.000574, 3.00043, 4.000574, 1.0);
    let mat2 = mat1.inverse2();
    assert_delta!(mat2.m11, 0.707107, 0.001);
    assert_delta!(mat2.m12, -0.331295, 0.001);
    assert_delta!(mat2.m13, 0.624695, 0.001);
    assert_delta!(mat2.m14, 0.0, 0.001);
    assert_delta!(mat2.m21, 0.0, 0.001);
    assert_delta!(mat2.m22, 0.883452, 0.001);
    assert_delta!(mat2.m23, 0.468521, 0.001);
    assert_delta!(mat2.m24, 0.0, 0.001);
    assert_delta!(mat2.m31, -0.707107, 0.001);
    assert_delta!(mat2.m32, -0.331295, 0.001);
    assert_delta!(mat2.m33, 0.624695, 0.001);
    assert_delta!(mat2.m34, 0.0, 0.001);
    assert_delta!(mat2.m41, 0.0, 0.001);
    assert_delta!(mat2.m42, 0.0, 0.001);
    assert_delta!(mat2.m43, -6.404043, 0.001);
    assert_delta!(mat2.m44, 1.0, 0.001);
}

#[test]
fn matrix44_can_be_multVecMatrix() {
}

#[test]
fn matrix44_can_be_multDirMatrix() {
}
