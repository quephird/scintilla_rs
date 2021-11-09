use crate::{material, matrix, ray, tuple};
use crate::float::EPSILON;
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Cube {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Cube {
    pub fn new(transform: Matrix4, material: Material) -> Cube {
        Cube {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = (-1. - origin);
    let tmax_numerator = (1. - origin);

    let mut tmin: f64 = 0.0;
    let mut tmax: f64 = 0.0;

    if direction.abs() >= EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * f64::INFINITY;
        tmax = tmax_numerator * f64::INFINITY;
    }

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Shape for Cube {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        let (xtmin, xtmax) = check_axis(local_ray.origin[0], local_ray.direction[0]);
        let (ytmin, ytmax) = check_axis(local_ray.origin[1], local_ray.direction[1]);
        let (ztmin, ztmax) = check_axis(local_ray.origin[2], local_ray.direction[2]);
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        vec![tmin, tmax]
    }

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        tuple::Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use crate::cube::{Cube, material};
    use crate::matrix;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::{Tuple, TupleMethods};

    #[test]
    fn test_intersect_outside() {
        let cube = Cube::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let test_cases = vec![
            (Tuple::point(5., 0.5, 0.), Tuple::vector(-1., 0., 0.)),
            (Tuple::point(-5., 0.5, 0.), Tuple::vector(1., 0., 0.)),
            (Tuple::point(0.5, 5., 0.), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0.5, -5.,  0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0.5, 0., 5.), Tuple::vector(0., 0., -1.)),
            (Tuple::point(0.5, 0., -5.), Tuple::vector(0., 0., 1.)),
        ];
        for (origin, direction) in test_cases {
            let ray = Ray::new(origin, direction);
            let ts = cube.intersect(&ray);
            assert_eq!(ts.len(), 2);
            assert_eq!(ts[0], 4.);
            assert_eq!(ts[1], 6.);
        }
    }

    #[test]
    fn test_intersect_inside() {
        let cube = Cube::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let ray = Ray::new(
            Tuple::point(0., 0.5, 0.),
            Tuple::vector(0., 0., 1.),
        );
        let ts = cube.intersect(&ray);
        assert_eq!(ts.len(), 2);
        assert_eq!(ts[0], -1.);
        assert_eq!(ts[1], 1.)
    }
}
