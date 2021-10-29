use crate::matrix;
use crate::tuple::TupleMethods;

pub fn translation(x: f64, y: f64, z: f64) -> matrix::Matrix4 {
    [
        [1., 0., 0., x],
        [0., 1., 0., y],
        [0., 0., 1., z],
        [0., 0., 0., 1.]
    ]
}

pub fn scaling(x: f64, y: f64, z: f64) -> matrix::Matrix4 {
    [
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., z, 0.],
        [0., 0., 0., 1.]
    ]
}

pub fn rotation_x(t: f64) -> matrix::Matrix4 {
    [
        [1.,      0.,       0., 0.],
        [0., t.cos(), -t.sin(), 0.],
        [0., t.sin(),  t.cos(), 0.],
        [0.,      0.,       0., 1.]
    ]
}

pub fn rotation_y(t: f64) -> matrix::Matrix4 {
    [
        [ t.cos(), 0., t.sin(), 0.],
        [      0., 1.,      0., 0.],
        [-t.sin(), 0., t.cos(), 0.],
        [      0., 0.,      0., 1.]
    ]
}

pub fn rotation_z(t: f64) -> matrix::Matrix4 {
    [
        [t.cos(), -t.sin(), 0., 0.],
        [t.sin(),  t.cos(), 0., 0.],
        [     0.,       0., 1., 0.],
        [     0.,       0., 0., 1.]
    ]
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> matrix::Matrix4 {
    [
        [1., xy, xz, 0.],
        [yx, 1., yz, 0.],
        [zx, zy, 1., 0.],
        [0., 0., 0., 1.]
    ]
}


#[cfg(test)]
mod tests {
    use crate::matrix;
    use std::f64::consts::PI;
    use super::*;

    #[test]
    fn test_translation() {
        let p = [-3., 4., 5., 1.];
        let t = translation(5., -3., 2.);
        let expected_value = [2., 1., 7., 1.];
        assert!(matrix::multiply_by_tuple(t, p).is_equal(expected_value));

        let t_inverse = matrix::inverse_4x4(t);
        let expected_value2 = [-8., 7., 3., 1.];
        assert!(matrix::multiply_by_tuple(t_inverse.unwrap(), p).is_equal(expected_value2));
    }

    #[test]
    fn test_scaling() {
        let p = [-4., 6., 8., 1.];
        let s = scaling(2., 3., 4.);
        let expected_value = [-8., 18., 32., 1.];
        assert!(matrix::multiply_by_tuple(s, p).is_equal(expected_value));

        let s_inverse = matrix::inverse_4x4(s);
        let expected_value2 = [-2., 2., 2., 1.];
        assert!(matrix::multiply_by_tuple(s_inverse.unwrap(), p).is_equal(expected_value2));
    }

    #[test]
    fn test_rotation_x() {
        let p = [0., 1., 0., 1.];
        let half_quarter = rotation_x(PI/4.0);
        let full_quarter = rotation_x(PI/2.0);

        let expected_value1 = [0., 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 1.];
        assert!(matrix::multiply_by_tuple(half_quarter, p).is_equal(expected_value1));

        let expected_value2 = [0., 0., 1., 1.];
        assert!(matrix::multiply_by_tuple(full_quarter, p).is_equal(expected_value2));
    }

    #[test]
    fn test_rotation_y() {
        let p = [0., 0., 1., 1.];
        let half_quarter = rotation_y(PI/4.0);
        let full_quarter = rotation_y(PI/2.0);

        let expected_value1 = [2.0_f64.sqrt()/2.0, 0., 2.0_f64.sqrt()/2.0, 1.];
        assert!(matrix::multiply_by_tuple(half_quarter, p).is_equal(expected_value1));

        let expected_value2 = [1., 0., 0., 1.];
        assert!(matrix::multiply_by_tuple(full_quarter, p).is_equal(expected_value2));
    }

    #[test]
    fn test_rotation_z() {
        let p = [0., 1., 0., 1.];
        let half_quarter = rotation_z(PI/4.0);
        let full_quarter = rotation_z(PI/2.0);

        let expected_value1 = [-2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0., 1.];
        assert!(matrix::multiply_by_tuple(half_quarter, p).is_equal(expected_value1));

        let expected_value2 = [-1., 0., 0., 1.];
        assert!(matrix::multiply_by_tuple(full_quarter, p).is_equal(expected_value2));
    }

    #[test]
    fn test_shearing() {
        let p = [2., 3., 4., 1.];
        let sxy = shearing(1., 0., 0., 0., 0., 0.);
        let expected_value1 = [5., 3., 4., 1.];
        assert!(matrix::multiply_by_tuple(sxy, p).is_equal(expected_value1));

        let sxz = shearing(0., 1., 0., 0., 0., 0.);
        let expected_value2 = [6., 3., 4., 1.];
        assert!(matrix::multiply_by_tuple(sxz, p).is_equal(expected_value2));

        let syx = shearing(0., 0., 1., 0., 0., 0.);
        let expected_value3 = [2., 5., 4., 1.];
        assert!(matrix::multiply_by_tuple(syx, p).is_equal(expected_value3));

        let syz = shearing(0., 0., 0., 1., 0., 0.);
        let expected_value4 = [2., 7., 4., 1.];
        assert!(matrix::multiply_by_tuple(syz, p).is_equal(expected_value4));

        let szx = shearing(0., 0., 0., 0., 1., 0.);
        let expected_value5 = [2., 3., 6., 1.];
        assert!(matrix::multiply_by_tuple(szx, p).is_equal(expected_value5));

        let szy = shearing(0., 0., 0., 0., 0., 1.);
        let expected_value6 = [2., 3., 7., 1.];
        assert!(matrix::multiply_by_tuple(szy, p).is_equal(expected_value6));
    }

    #[test]
    fn test_chained_transformations() {
        let p = [1., 0., 1., 1.];
        let r = rotation_x(PI/2.);
        let s = scaling(5., 5., 5.);
        let t = translation(10., 5., 7.);
        let tsr = matrix::multiply_by_matrix(t, matrix::multiply_by_matrix(s, r));
        let expected_value = [15., 0., 7., 1.];
        assert!(matrix::multiply_by_tuple(tsr, p).is_equal(expected_value));
    }
}
