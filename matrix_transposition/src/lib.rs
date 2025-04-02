#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_test() {
        let m = Matrix((1, 2), (3, 4));
        let result = transpose(m);
        assert_eq!(result, Matrix((1, 3), (2, 4)));
    }
}
