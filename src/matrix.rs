use crate::float;
use crate::tuple;
use crate::tuple::TupleMethods;

type Matrix2 = [[f64; 2]; 2];
type Matrix3 = [[f64; 3]; 3];
pub type Matrix4 = [[f64; 4]; 4];

pub const IDENTITY: Matrix4 = [
    [1., 0., 0., 0.],
    [0., 1., 0., 0.],
    [0., 0., 1., 0.],
    [0., 0., 0., 1.]
];

pub fn is_equal(m1: Matrix4, m2: Matrix4) -> bool {
    for r in 0..4 {
        for c in 0..4 {
            if !float::is_equal(m1[r][c], m2[r][c]) {
                return false
            }
        }
    }
    true
}

pub fn multiply_by_matrix(m1: Matrix4, m2: Matrix4) -> Matrix4 {
    let mut m3: Matrix4 = [[0.; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            m3[r][c] = m1[r].dot([m2[0][c], m2[1][c], m2[2][c], m2[3][c]]);
        }
    }
    m3
}

pub fn multiply_by_tuple(m: Matrix4, t: tuple::Tuple) -> tuple::Tuple {
    let mut t2: tuple::Tuple = [0.; 4];
    for r in 0..4 {
        t2[r] = m[r].dot(t);
    }
    t2
}

pub fn transpose(m: Matrix4) -> Matrix4 {
    let mut m2: Matrix4 = [[0.; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            m2[r][c] = m[c][r];
        }
    }
    m2
}

pub fn determinant_2x2(m: Matrix2) -> f64 {
    m[0][0]*m[1][1] - m[0][1]*m[1][0]
}

pub fn submatrix_3x3(m: Matrix3, i: usize, j: usize) -> Matrix2 {
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
            m2[r2][c2] = m[r][c];
            c2 += 1;
        }
        r2 +=1;
    }
    m2
}

pub fn minor_3x3(m: Matrix3, i: usize, j: usize) -> f64 {
    determinant_2x2(submatrix_3x3(m, i, j))
}

pub fn cofactor_3x3(m: Matrix3, i: usize, j: usize) -> f64 {
    let minor = minor_3x3(m, i, j);
    if (i+j) % 2 == 0 {
        minor
    } else {
        -minor
    }
}

pub fn determinant_3x3(m: Matrix3) -> f64 {
    let mut d = 0.;
    for i in 0..3 {
        d += m[0][i]*cofactor_3x3(m, i, 0);
    }
    d
}

pub fn submatrix_4x4(m: Matrix4, i: usize, j: usize) -> Matrix3 {
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
            m2[r2][c2] = m[r][c];
            c2 += 1;
        }
        r2 +=1;
    }
    m2
}

pub fn minor_4x4(m: Matrix4, i: usize, j: usize) -> f64 {
    determinant_3x3(submatrix_4x4(m, i, j))
}

pub fn cofactor_4x4(m: Matrix4, i: usize, j: usize) -> f64 {
    let minor = minor_4x4(m, i, j);
    if (i+j) % 2 == 0 {
        minor
    } else {
        -minor
    }
}

pub fn determinant_4x4(m: Matrix4) -> f64 {
    let mut d = 0.;
    for i in 0..4 {
        d += m[0][i]*cofactor_4x4(m, i, 0);
    }
    d
}

pub fn inverse_4x4(m: Matrix4) -> Option<Matrix4> {
    let d = determinant_4x4(m);
    if d == 0. {
        None
    } else {
        let mut m2: Matrix4 = [[0.; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                m2[c][r] = cofactor_4x4(m, c, r)/d;
            }
        }
        Some(m2)
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
        assert_eq!(is_equal(m1, m2), true);

        let m3 = [
            [2., 3., 4., 5.],
            [6., 7., 8., 9.],
            [8., 7., 6., 5.],
            [4., 3., 2., 1.]
        ];
        assert_eq!(is_equal(m1, m3), false);
    }

    #[test]
    fn test_multiply_by_matrix() {
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
        assert_eq!(is_equal(multiply_by_matrix(m1, m2), expected_value), true);

        let m3 = [
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.]
        ];
        assert_eq!(is_equal(multiply_by_matrix(m3, IDENTITY), m3), true);
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
        assert!(multiply_by_tuple(m, t).is_equal(expected_value));
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
        assert_eq!(is_equal(transpose(m), expected_value), true);
    }

    #[test]
    fn test_determinant_2x2() {
        let m = [
            [1., 5.],
            [-3., 2.]
        ];
        assert_eq!(float::is_equal(determinant_2x2(m), 17.), true);
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
        assert_eq!(submatrix_3x3(m, 2, 0), expected_value);
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
        assert_eq!(submatrix_4x4(m, 1, 2), expected_value);
    }

    #[test]
    fn test_minor_3x3() {
        let m = [
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        ];
        assert_eq!(float::is_equal(minor_3x3(m, 0, 1), 25.), true);
    }

    #[test]
    fn test_cofactor_3x3() {
        let m = [
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        ];
        assert_eq!(float::is_equal(cofactor_3x3(m, 0, 0), -12.), true);
        assert_eq!(float::is_equal(cofactor_3x3(m, 0, 1), -25.), true);
    }

    #[test]
    fn test_determinant_3x3() {
        let m = [
            [1., 2., 6.],
            [-5., 8., -4.],
            [2., 6., 4.]
        ];
        assert_eq!(float::is_equal(determinant_3x3(m), -196.), true);
    }

    #[test]
    fn test_determinant_4x4() {
        let m = [
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.]
        ];
        assert_eq!(float::is_equal(determinant_4x4(m), -4071.), true);
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
        assert_eq!(is_equal(inverse_4x4(m).unwrap(), expected_value), true);

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
        let c = multiply_by_matrix(a, b);
        assert_eq!(is_equal(multiply_by_matrix(c, inverse_4x4(b).unwrap()), a), true);
    }
}
