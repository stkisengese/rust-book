use lalgebra_scalar::Scalar;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + Default> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::default()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut identity_matrix = vec![vec![T::default(); n]; n];
        for i in 0..n {
            identity_matrix[i][i] = T::one(); // Set diagonal elements to 1
        }
        Matrix(identity_matrix)
	}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_matrix () {
//         let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
//         let result1 = Matrix::<i32>::identity(4);
//         let result2 = Matrix::<f64>::zero(3, 4);
//         assert_eq!(m, Matrix([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]));
//         assert_eq!(result1, Matrix([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]));
//         // assert_eq!(result2, Matrix([[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]]));
//     }
// }
