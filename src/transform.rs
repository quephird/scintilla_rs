use crate::{matrix, transform};
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::tuple::{Tuple, TupleMethods};

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

pub fn view(from: Tuple, to: Tuple, up: Tuple) -> Matrix4 {
    let forward = to.subtract(from).normalize();
    let up_normalized = up.normalize();
    let left = forward.cross(up_normalized);
    let true_up = left.cross(forward);
    let orientation = [
        left,
        true_up,
        forward.negate(),
        [0., 0., 0., 1.],
    ];
    let backward_transform = transform::translation(-from[0], -from[1], -from[2]);
    orientation.multiply_matrix(backward_transform)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::matrix::Matrix4Methods;
    use super::*;

    #[test]
    fn test_translation() {
        let p = [-3., 4., 5., 1.];
        let t = translation(5., -3., 2.);
        let expected_value = [2., 1., 7., 1.];
        assert!(t.multiply_tuple(p).is_equal(expected_value));

        let t_inverse = t.inverse();
        let expected_value2 = [-8., 7., 3., 1.];
        assert!(t_inverse.unwrap().multiply_tuple(p).is_equal(expected_value2));
    }

    #[test]
    fn test_scaling() {
        let p = [-4., 6., 8., 1.];
        let s = scaling(2., 3., 4.);
        let expected_value = [-8., 18., 32., 1.];
        assert!(s.multiply_tuple(p).is_equal(expected_value));

        let s_inverse = s.inverse();
        let expected_value2 = [-2., 2., 2., 1.];
        assert!(s_inverse.unwrap().multiply_tuple(p).is_equal(expected_value2));
    }

    #[test]
    fn test_rotation_x() {
        let p = [0., 1., 0., 1.];
        let half_quarter = rotation_x(PI/4.0);
        let full_quarter = rotation_x(PI/2.0);

        let expected_value1 = [0., 2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 1.];
        assert!(half_quarter.multiply_tuple(p).is_equal(expected_value1));

        let expected_value2 = [0., 0., 1., 1.];
        assert!(full_quarter.multiply_tuple(p).is_equal(expected_value2));
    }

    #[test]
    fn test_rotation_y() {
        let p = [0., 0., 1., 1.];
        let half_quarter = rotation_y(PI/4.0);
        let full_quarter = rotation_y(PI/2.0);

        let expected_value1 = [2.0_f64.sqrt()/2.0, 0., 2.0_f64.sqrt()/2.0, 1.];
        assert!(half_quarter.multiply_tuple(p).is_equal(expected_value1));

        let expected_value2 = [1., 0., 0., 1.];
        assert!(full_quarter.multiply_tuple(p).is_equal(expected_value2));
    }

    #[test]
    fn test_rotation_z() {
        let p = [0., 1., 0., 1.];
        let half_quarter = rotation_z(PI/4.0);
        let full_quarter = rotation_z(PI/2.0);

        let expected_value1 = [-2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0., 1.];
        assert!(half_quarter.multiply_tuple(p).is_equal(expected_value1));

        let expected_value2 = [-1., 0., 0., 1.];
        assert!(full_quarter.multiply_tuple(p).is_equal(expected_value2));
    }

    #[test]
    fn test_shearing() {
        let p = [2., 3., 4., 1.];
        let sxy = shearing(1., 0., 0., 0., 0., 0.);
        let expected_value1 = [5., 3., 4., 1.];
        assert!(sxy.multiply_tuple(p).is_equal(expected_value1));

        let sxz = shearing(0., 1., 0., 0., 0., 0.);
        let expected_value2 = [6., 3., 4., 1.];
        assert!(sxz.multiply_tuple(p).is_equal(expected_value2));

        let syx = shearing(0., 0., 1., 0., 0., 0.);
        let expected_value3 = [2., 5., 4., 1.];
        assert!(syx.multiply_tuple(p).is_equal(expected_value3));

        let syz = shearing(0., 0., 0., 1., 0., 0.);
        let expected_value4 = [2., 7., 4., 1.];
        assert!(syz.multiply_tuple(p).is_equal(expected_value4));

        let szx = shearing(0., 0., 0., 0., 1., 0.);
        let expected_value5 = [2., 3., 6., 1.];
        assert!(szx.multiply_tuple(p).is_equal(expected_value5));

        let szy = shearing(0., 0., 0., 0., 0., 1.);
        let expected_value6 = [2., 3., 7., 1.];
        assert!(szy.multiply_tuple(p).is_equal(expected_value6));
    }

    #[test]
    fn test_chained_transformations() {
        let p = [1., 0., 1., 1.];
        let r = rotation_x(PI/2.);
        let s = scaling(5., 5., 5.);
        let t = translation(10., 5., 7.);
        let tsr = t.multiply_matrix(s.multiply_matrix(r));
        let expected_value = [15., 0., 7., 1.];
        assert!(tsr.multiply_tuple(p).is_equal(expected_value));
    }

    #[test]
    fn test_view_default() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., -1.);
        let up = Tuple::vector(0., 1., 0.);
        let view = view(from, to, up);
        assert!(view.is_equal(matrix::IDENTITY));
    }

    #[test]
    fn test_view_looking_in_positive_z_direction() {
        let from = Tuple::point(0., 0., 0.);
        let to = Tuple::point(0., 0., 1.);
        let up = Tuple::vector(0., 1., 0.);
        let view = view(from, to, up);
        let expected_value = transform::scaling(-1., 1., -1.);
        assert!(view.is_equal(expected_value));
    }

    #[test]
    fn test_view_moves_world() {
        let from = Tuple::point(0., 0., 8.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        let view = view(from, to, up);
        let expected_value = transform::translation(0., 0., -8.);
        assert!(view.is_equal(expected_value));
    }

    #[test]
    fn test_view_arbitrary() {
        let from = Tuple::point(1., 3., 2.);
        let to = Tuple::point(4., -2., 8.);
        let up = Tuple::vector(1., 1., 0.);
        let view = view(from, to, up);
        let expected_value = [
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ];
        assert!(view.is_equal(expected_value));
    }
}
