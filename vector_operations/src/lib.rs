#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

use std::ops::{Add, Sub};

impl<T: Add<Output = T>> Add for ThreeDVector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T: Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let a = ThreeDVector { i: 3, j: 5, k: 2 };
        let b = ThreeDVector { i: 2, j: 7, k: 4 };

        assert_eq!(a + b, ThreeDVector { i: 5, j: 12, k: 6 });
    }

    #[test]
    fn test_sub() {
        let a = ThreeDVector { i: 3, j: 5, k: 2 };
        let b = ThreeDVector { i: 2, j: 7, k: 4 };

        assert_eq!(a - b, ThreeDVector { i: 1, j: -2, k: -2 });
    }
}