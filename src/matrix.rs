use crate::float;
use crate::tuple;
use crate::tuple::TupleMethods;

type Matrix2 = [[f64; 2]; 2];

trait Matrix2Methods {
    fn determinant(&self) -> f64;
}

impl Matrix2Methods for Matrix2 {
    fn determinant(&self) -> f64 {
        self[0][0]*self[1][1] - self[0][1]*self[1][0]
    }
}

type Matrix3 = [[f64; 3]; 3];

trait Matrix3Methods {
    fn submatrix(&self, i: usize, j: usize) -> Matrix2;
    fn minor(&self, i: usize, j: usize) -> f64;
    fn cofactor(&self, i: usize, j: usize) -> f64;
    fn determinant(&self) -> f64;
}

impl Matrix3Methods for Matrix3 {
    fn submatrix(&self, i: usize, j: usize) -> Matrix2 {
        let mut m2: Matrix2 = [[0.; 2]; 2];
        let mut r2 = 0;
        for r in 0..3 {
            let mut c2 = 0;
            if r == j {
                continue;
            }
            for c in 0..3 {
                if c == i {
                    continue;
                }
                m2[r2][c2] = self[r][c];
                c2 += 1;
            }
            r2 +=1;
        }
        m2
    }

    fn determinant(&self) -> f64 {
        let mut d = 0.;
        for i in 0..3 {
            d += self[0][i]*(*self).cofactor(i, 0);
        }
        d
    }

    fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    fn cofactor(&self, i: usize, j: usize) -> f64 {
        let minor = self.minor(i, j);
        if (i+j) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

pub type Matrix4 = [[f64; 4]; 4];

pub const IDENTITY: Matrix4 = [
    [1., 0., 0., 0.],
    [0., 1., 0., 0.],
    [0., 0., 1., 0.],
    [0., 0., 0., 1.]
];

pub trait Matrix4Methods {
    fn is_equal(&self, other: Matrix4) -> bool;
    fn multiply_matrix(&self, other: Matrix4) -> Matrix4;
    fn multiply_tuple(&self, t: tuple::Tuple) -> tuple::Tuple;
    fn transpose(&self) -> Matrix4;
    fn submatrix(&self, i: usize, j: usize) -> Matrix3;
    fn minor(&self, i: usize, j: usize) -> f64;
    fn cofactor(&self, i: usize, j: usize) -> f64;
    fn determinant(&self) -> f64;
    fn inverse(&self) -> Option<Matrix4>;
}

impl Matrix4Methods for Matrix4 {
    fn is_equal(&self, other: Matrix4) -> bool {
        for row in 0..4 {
            if !self[row].is_equal(other[row]) {
                return false
            }
        }
        true
    }

    fn multiply_matrix(&self, other: Matrix4) -> Matrix4 {
        let mut m: Matrix4 = [[0.; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                m[r][c] = self[r].dot([other[0][c], other[1][c], other[2][c], other[3][c]]);
            }
        }
        m
    }

    fn multiply_tuple(&self, t: tuple::Tuple) -> tuple::Tuple {
        let mut t2: tuple::Tuple = [0.; 4];
        for r in 0..4 {
            t2[r] = self[r].dot(t);
        }
        t2
    }

    fn transpose(&self) -> Matrix4 {
        let mut m: Matrix4 = [[0.; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                m[r][c] = self[c][r];
            }
        }
        m
    }

    fn submatrix(&self, i: usize, j: usize) -> Matrix3 {
        let mut m2: Matrix3 = [[0.; 3]; 3];
        let mut r2 = 0;
        for r in 0..4 {
            let mut c2 = 0;
            if r == j {
                continue;
            }
            for c in 0..4 {
                if c == i {
                    continue;
                }
                m2[r2][c2] = self[r][c];
                c2 += 1;
            }
            r2 +=1;
        }
        m2
    }

    fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    fn cofactor(&self, i: usize, j: usize) -> f64 {
        let minor = self.minor(i, j);
        if (i+j) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(&self) -> f64 {
        let mut d = 0.;
        for i in 0..4 {
            d += self[0][i]*(*self).cofactor(i, 0);
        }
        d
    }

    fn inverse(&self) -> Option<Matrix4> {
        let d = self.determinant();
        if d == 0. {
            None
        } else {
            let mut m2: Matrix4 = [[0.; 4]; 4];
            for r in 0..4 {
                for c in 0..4 {
                    m2[c][r] = (*self).cofactor(c, r)/d;
                }
            }
            Some(m2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::TupleMethods;
    use super::*;

    #[test]
    fn test_is_equal() {
        let m1 = [
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ];
        let m2 = [
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ];
        assert!(m1.is_equal(m2));

        let m3 = [
            [2., 3., 4., 5.],
            [6., 7., 8., 9.],
            [8., 7., 6., 5.],
            [4., 3., 2., 1.]
        ];
        assert!(!m1.is_equal(m3));
    }

    #[test]
    fn test_multiply_matrix() {
        let m1 = [
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ];
        let m2 = [
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.]
        ];
        let expected_value = [
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.]
        ];
        assert!(m1.multiply_matrix(m2).is_equal(expected_value));

        let m3 = [
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.]
        ];
        assert!(m3.multiply_matrix(IDENTITY).is_equal(m3));
    }

    #[test]
    fn test_multiply_by_tuple() {
        let m = [
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.]
        ];
        let t = [1., 2., 3., 1.];
        let expected_value = [18., 24., 33., 1.];
        assert!(m.multiply_tuple(t).is_equal(expected_value));
    }

    #[test]
    fn test_transpose() {
        let m = [
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.]
        ];
        let expected_value = [
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.]
        ];
        assert!(m.transpose().is_equal(expected_value));
    }

    #[test]
    fn test_determinant_2x2() {
        let m = [
            [1., 5.],
            [-3., 2.]
        ];
        assert!(float::is_equal(m.determinant(), 17.));
    }

    #[test]
    fn test_submatrix_3x3() {
        let m = [
            [1., 5., 0.],
            [-3., 2., 7.],
            [0., 6., -3.]
        ];
        let expected_value = [
            [-3., 2.],
            [0., 6.]
        ];
        assert_eq!(m.submatrix(2, 0), expected_value);
    }

    #[test]
    fn test_submatrix_4x4() {
        let m = [
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.]
        ];
        let expected_value = [
            [-6., 1., 6.],
            [-8., 8., 6.],
            [-7., -1., 1.]
        ];
        assert_eq!(m.submatrix(1, 2), expected_value);
    }

    #[test]
    fn test_minor_3x3() {
        let m = [
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        ];
        assert!(float::is_equal(m.minor(0, 1), 25.));
    }

    #[test]
    fn test_cofactor_3x3() {
        let m = [
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        ];
        assert!(float::is_equal(m.cofactor(0, 0), -12.));
        assert!(float::is_equal(m.cofactor(0, 1), -25.));
    }

    #[test]
    fn test_determinant_3x3() {
        let m = [
            [1., 2., 6.],
            [-5., 8., -4.],
            [2., 6., 4.]
        ];
        assert!(float::is_equal(m.determinant(), -196.));
    }

    #[test]
    fn test_determinant_4x4() {
        let m = [
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.]
        ];
        assert!(float::is_equal(m.determinant(), -4071.));
    }

    #[test]
    fn test_inverse_4x4() {
        let m = [
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.]
        ];
        let expected_value = [
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]
        ];
        assert!(m.inverse().unwrap().is_equal(expected_value));

        let a = [
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.]
        ];
        let b = [
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.]
        ];
        let c = a.multiply_matrix(b);
        assert!(c.multiply_matrix(b.inverse().unwrap()).is_equal(a));
    }
}
