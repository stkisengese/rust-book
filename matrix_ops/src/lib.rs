use std::ops::{Add, Sub};
// use matrix::Matrix;
use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result = self.0.iter()
           .zip(other.0.iter())
           .map(|(row1, row2)| {
                row1.iter()
                .zip(row2.iter())
                .map(|(a, b)| *a + *b)
                .collect()
        }).collect();

        Some(Matrix(result))
    }
}

impl<T: Scalar<Item = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result = self.0.iter()
          .zip(other.0.iter())
          .map(|(row1, row2)| {
                row1.iter()
                .zip(row2.iter())
                .map(|(a, b)| *a - *b)
                .collect()
        }).collect();

        Some(Matrix(result))
    }
}