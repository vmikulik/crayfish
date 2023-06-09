use crate::Result;
use crate::constants::EPSILON;
use crate::tuples::{Tuple, TupleType};
use crate::eq;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    contents: Vec<f64>,
}


impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, _index: (usize, usize)) -> &f64 {
        match _index {
            (i, j) => &self.contents[i * self.width + j],
        }
    }
}


impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, _index: (usize, usize)) -> &mut f64 {
        match _index {
            (i, j) => &mut self.contents[i * self.width + j],
        }
    }
}


/// `/` means Matrix multiplication.
impl std::ops::Div<&Matrix> for Matrix {
    type Output = Matrix;

    fn div(self, _rhs: &Matrix) -> Matrix {
        self.matmul(_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl std::ops::Div<Matrix> for Matrix {
    type Output = Matrix;

    fn div(self, _rhs: Matrix) -> Matrix {
        self.matmul(&_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl<K: TupleType<K>> std::ops::Div<&Tuple<K>> for Matrix {
    type Output = Tuple<K>;

    fn div(self, _rhs: &Tuple<K>) -> Tuple<K> {
        self.matmul_t(_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl<K: TupleType<K>> std::ops::Div<Tuple<K>> for Matrix {
    type Output = Tuple<K>;

    fn div(self, _rhs: Tuple<K>) -> Tuple<K> {
        self.matmul_t(&_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl std::ops::Div<&Matrix> for &Matrix {
    type Output = Matrix;

    fn div(self, _rhs: &Matrix) -> Matrix {
        self.matmul(_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl std::ops::Div<Matrix> for &Matrix {
    type Output = Matrix;

    fn div(self, _rhs: Matrix) -> Matrix {
        self.matmul(&_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl<K: TupleType<K>> std::ops::Div<&Tuple<K>> for &Matrix {
    type Output = Tuple<K>;

    fn div(self, _rhs: &Tuple<K>) -> Tuple<K> {
        self.matmul_t(_rhs).unwrap()
    }
}

/// `/` means Matrix multiplication.
impl<K: TupleType<K>> std::ops::Div<Tuple<K>> for &Matrix {
    type Output = Tuple<K>;

    fn div(self, _rhs: Tuple<K>) -> Tuple<K> {
        self.matmul_t(&_rhs).unwrap()
    }
}


impl std::cmp::PartialEq for Matrix {
    fn eq(&self, _rhs: &Matrix) -> bool {
        if self.width != _rhs.width || self.height != _rhs.height {
            return false;
        }
        for i in 0..self.height {
            for j in 0..self.width {
                if (self[(i,j)] - _rhs[(i,j)]).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Matrix {
        Matrix {
            height,
            width,
            contents: vec![0.; width * height],
        }
    }

    pub fn from_rows(rows: &Vec<Vec<f64>>) -> Result<Matrix> {
        let height = rows.len();
        let widths: Vec<usize> = rows.iter().map(|row| row.len()).collect();
        let width = widths[0];
        if widths.iter().any(|w| *w != width) {
            return Err("All rows must have the same length".into());
        }
        let mut m = Matrix::new(width, height);
        for i in 0..height {
            for j in 0..width {
                m[(i,j)] = rows[i][j];
            }
        }
        Ok(m)
    }

    pub fn from_cols(cols: &Vec<Vec<f64>>) -> Result<Matrix> {
        let width = cols.len();
        let heights: Vec<usize> = cols.iter().map(|col| col.len()).collect();
        let height = heights[0];
        if heights.iter().any(|h| *h != height) {
            return Err("All columns must have the same length".into());
        }
        let mut m = Matrix::new(width, height);
        for i in 0..height {
            for j in 0..width {
                m[(i,j)] = cols[j][i];
            }
        }
        Ok(m)
    }

    pub fn identity(size: usize) -> Matrix {
        let mut result = Matrix::new(size, size);
        for i in 0..size {
            result[(i,i)] = 1.;
        }
        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }

    pub fn matmul(&self, _rhs: &Matrix) -> Result<Matrix> {
        if self.width != _rhs.height {
            return Err("Matrix dimensions must agree".into());
        }
        let height = self.height;
        let width = _rhs.width;
        let inner = self.width;
        let mut result = Matrix::new(height, width);
        for i in 0..height {
            for j in 0..width {
                for k in 0..inner {
                    result[(i,j)] += self[(i,k)] * _rhs[(k,j)];
                }
            }
        }
        Ok(result)
    }

    /// Matmul by tuple.
    ///
    /// The tuple is implicitly 4-d, with a 4th element
    /// denoting whether it's a vector (if 0) or a point (if 1).
    /// This means the bottom row of our matrices is assumed
    /// to be 0, 0, 0, 1. and the rightmost column
    /// is essentially the translation vector, which
    /// only gets applied to points.
    pub fn matmul_t<K: TupleType<K>>(&self, _rhs: &Tuple<K>) -> Result<Tuple<K>> {
        if self.width != 4 || self.height != 4 {
            return Err("Matrix must be 4x4".into());
        }
        let rhs_contents = _rhs.as_array();
        let mut out: [f64; 3] = [0.0; 3];
        // If it's a vector, we don't bother looking at the
        // rightmost column as it would be multiplied by 0.
        let width = if rhs_contents[3] == 0.0 {3} else {4};
        // We don't care about the bottom row in any case.
        for i in 0..self.height-1 {
            for j in 0..width {
                out[i] += self[(i,j)] * rhs_contents[j]
            }
        }

        Ok(Tuple::new(
            out[0],
            out[1],
            out[2],
        ))
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Result<Matrix> {
        if row >= self.height || col >= self.width {
            return Err("Submatrix indices out of bounds".into());
        }
        let mut result = Matrix::new(self.height - 1, self.width - 1);
        for i in 0..self.height {
            for j in 0..self.width {
                if i != row && j != col {
                    let new_i = if i > row { i - 1 } else { i };
                    let new_j = if j > col { j - 1 } else { j };
                    result[(new_i,new_j)] = self[(i,j)];
                }
            }
        }
        Ok(result)
    }

    pub fn is_invertible(&self) -> Result<bool> {
        Ok(self.height == self.width && !eq(det(self)?, 0.))
    }

    pub fn inverse(&self) -> Result<Matrix> {
        if !self.is_invertible()? {
            return Err("Matrix is not invertible".into());
        }

        let mut result = Matrix::new(self.height, self.width);
        let det = det(self)?;

        for i in 0..self.height {
            for j in 0..self.width {
                result[(j,i)] = cofactor(self, i, j)? / det;
            }
        }
        Ok(result)
    }


}


pub fn det(m: &Matrix) -> Result<f64> {
    if m.height != m.width {
        return Err("Matrix must be square".into());
    }
    if m.height == 1 {
        return Ok(m[(0,0)]);
    }
    if m.height == 2 {
        return Ok(m[(0,0)] * m[(1,1)]
                  - m[(0,1)] * m[(1,0)]);
    }
    let mut result = 0.;
    for i in 0..m.width {
        let submatrix = m.submatrix(0, i)?;
        let sign = if i % 2 == 0 { 1. } else { -1. };
        result += sign * m[(0,i)] * det(&submatrix)?;
    }
    Ok(result)

}


fn cofactor(m: &Matrix, row: usize, col: usize) -> Result<f64> {
    let submatrix = m.submatrix(row, col)?;
    let sign = if (row + col) % 2 == 0 { 1. } else { -1. };
    Ok(sign * det(&submatrix)?)
}


#[cfg(test)]
mod tests {
    use super::*;
    use proptest_strategies::matrix_4x4;
    use proptest::prelude::*;

    proptest! {

        #[test]
        fn matmul_4x4_by_inverse_gives_identity(
            a in matrix_4x4(100.),
        ) {
            prop_assume!(a.is_invertible().unwrap());
            prop_assert_eq!(&a / &a.inverse().unwrap(), Matrix::identity(4))
        }
    }


    #[test]
    fn two_by_two_matmul() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1., 2.], vec![3., 4.]])?;
        let b = Matrix::from_rows(&vec![vec![5., 6.], vec![7., 8.]])?;
        let c = Matrix::from_rows(&vec![vec![19., 22.], vec![43., 50.]])?;
        assert_eq!(a.matmul(&b)?, c);
        Ok(())
    }

    #[test]
    fn matmul_t() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.]
        ])?;
        let b = Tuple::point(1., 2., 3.);
        let c = Tuple::point(18., 24., 33.);
        assert_eq!(a.matmul_t(&b)?, c);
        Ok(())
    }

    #[test]
    fn matmul_via_div() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1., 2.], vec![3., 4.]])?;
        let b = Matrix::from_rows(&vec![vec![5., 6.], vec![7., 8.]])?;
        let c = Matrix::from_rows(&vec![vec![19., 22.], vec![43., 50.]])?;
        assert_eq!(a/b, c);
        Ok(())
    }

    #[test]
    fn chained_matmul_via_div() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1., 2.], vec![3., 4.]])?;
        let b = Matrix::from_rows(&vec![vec![5., 6.], vec![7., 8.]])?;
        assert_eq!(&a/&b/((a/b).inverse())?, Matrix::identity(2));
        Ok(())
    }

    #[test]
    fn transpose_2x2() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1., 2.], vec![3., 4.]])?;
        let b = Matrix::from_rows(&vec![vec![1., 3.], vec![2., 4.]])?;
        assert_eq!(a.transpose(), b);
        Ok(())
    }

    #[test]
    fn transpose_4x4() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 2., 3., 4.,],
            vec![5., 6., 7., 8.,],
            vec![9., 8., 7., 6.,],
            vec![5., 4., 3., 2.,],
        ])?;
        let b = Matrix::from_rows(&vec![
            vec![1., 5., 9., 5.,],
            vec![2., 6., 8., 4.,],
            vec![3., 7., 7., 3.,],
            vec![4., 8., 6., 2.,],
        ])?;
        assert_eq!(a.transpose(), b);
        Ok(())
    }

    #[test]
    fn identity_4x4() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 2., 3., 4.,],
            vec![2., 4., 4., 2.,],
            vec![8., 6., 4., 1.,],
            vec![0., 0., 0., 1.,],
        ])?;
        let b = Matrix::from_rows(&vec![
            vec![1., 2., 3., 4.,],
            vec![2., 4., 4., 2.,],
            vec![8., 6., 4., 1.,],
            vec![0., 0., 0., 1.,],
        ])?;
        assert_eq!(a.matmul(&Matrix::identity(4))?, b);
        Ok(())
    }

    #[test]
    fn identity_2x2() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1., 2.], vec![3., 4.]])?;
        let b = Matrix::from_rows(&vec![vec![1., 2.], vec![3., 4.]])?;
        assert_eq!(a.matmul(&Matrix::identity(2))?, b);
        Ok(())
    }

    #[test]
    fn submatrix_2x2() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1., 5.], vec![-3., 2.]])?;
        let b = Matrix::from_rows(&vec![vec![-3.]])?;
        assert_eq!(a.submatrix(0, 1)?, b);
        Ok(())
    }

    #[test]
    fn submatrix_3x3() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 5., 0.],
            vec![-3., 2., 7.],
            vec![0., 6., -3.],
        ])?;
        let b = Matrix::from_rows(&vec![
            vec![-3., 2.],
            vec![0., 6.],
        ])?;
        assert_eq!(a.submatrix(0, 2)?, b);
        Ok(())
    }

    #[test]
    fn submatrix_4x4() -> Result<()> {
        let a = Matrix::from_rows(
            &vec![
                vec![-6., 1., 1., 6.],
                vec![-8., 5., 8., 6.],
                vec![-1., 0., 8., 2.],
                vec![-7., 1., -1., 1.],
            ]
        )?;
        let b = Matrix::from_rows(
            &vec![
                vec![-6., 1., 6.],
                vec![-8., 8., 6.],
                vec![-7., -1., 1.],
            ]
        )?;
        assert_eq!(a.submatrix(2, 1)?, b);
        Ok(())
    }

    #[test]
    fn determinant_1x1() -> Result<()> {
        let a = Matrix::from_rows(&vec![vec![1.]])?;
        assert_eq!(det(&a)?, 1.);
        Ok(())
    }

    #[test]
    fn determinant_2x2() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 5.],
            vec![-3., 2.],
        ])?;
        assert_eq!(det(&a)?, 17.);
        Ok(())
    }

    #[test]
    fn determinant_3x3() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 2., 6.],
            vec![-5., 8., -4.],
            vec![2., 6., 4.],
        ])?;
        assert_eq!(det(&a)?, -196.);
        Ok(())
    }

    #[test]
    fn determinant_4x4() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![-2., -8., 3., 5.],
            vec![-3., 1., 7., 3.],
            vec![1., 2., -9., 6.],
            vec![-6., 7., 7., -9.],
        ])?;
        assert_eq!(det(&a)?, -4071.);
        Ok(())
    }

    #[test]
    fn inverse_4x4() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![-2., -8., 3., 5.],
            vec![-3., 1., 7., 3.],
            vec![1., 2., -9., 6.],
            vec![-6., 7., 7., -9.],
        ])?;
        assert_eq!(a.inverse()?.matmul(&a)?, Matrix::identity(4));
        Ok(())
    }

    #[test]
    fn inverse_2x2() -> Result<()> {
        let a = Matrix::from_rows(&vec![
            vec![1., 5.],
            vec![-3., 2.],
        ])?;
        assert_eq!(a.inverse()? / a, Matrix::identity(2));
        Ok(())
    }


}


#[cfg(test)]
pub mod proptest_strategies {
    use proptest::prelude::*;
    use super::Matrix;

    pub fn matrix_4x4(max_val: f64) -> impl Strategy<Value = Matrix> {
        (
            prop::collection::vec(-max_val..max_val, 4),
            prop::collection::vec(-max_val..max_val, 4),
            prop::collection::vec(-max_val..max_val, 4),
            prop::collection::vec(-max_val..max_val, 4),
        ).prop_map(|(r1, r2, r3, r4)|
            Matrix::from_rows(&vec![r1, r2, r3, r4]).unwrap()
        )
    }
}