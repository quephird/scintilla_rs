use crate::{material, matrix, ray, tuple};
use crate::float::EPSILON;
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::TupleMethods;

#[derive(Clone)]
pub struct Plane {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Plane {
    pub fn new(transform: Matrix4, material: Material) -> Plane {
        Plane {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        if local_ray.direction[1].abs() < EPSILON {
            vec![]
        } else {
            vec![-local_ray.origin[1] / local_ray.direction[1]]
        }
    }

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        tuple::Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use crate::{float, material, matrix};
    use crate::plane::Plane;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::{Tuple, TupleMethods};

    #[test]
    fn test_normal_at() {
        let plane = Plane::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL
        );
        let n1 = plane.normal_at(Tuple::point(0., 0., 0.));
        let n2 = plane.normal_at(Tuple::point(10., 0., -10.));
        let n3 = plane.normal_at(Tuple::point(-5., 0., 150.));
        assert!(
            vec![n1, n2, n3]
                .iter()
                .all(|v| v.is_equal(Tuple::vector(0., 1., 0.))));
    }

    #[test]
    fn test_intersect_parallel_ray() {
        let plane = Plane::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL
        );
        let local_ray= Ray::new(
            Tuple::point(0., 10., 0.),
            Tuple::vector(0., 0., 1.)
        );
        let ts = plane.intersect(&local_ray);
        assert_eq!(ts.len(), 0);
    }

    fn test_intersect_coplanar_ray() {
        let plane = Plane::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL
        );
        let local_ray= Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.)
        );
        let ts = plane.intersect(&local_ray);
        assert_eq!(ts.len(), 0);
    }

    #[test]
    fn test_intersect_above() {
        let plane = Plane::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL
        );
        let local_ray= Ray::new(
            Tuple::point(0., 1., 0.),
            Tuple::vector(0., -1., 0.)
        );
        let ts = plane.intersect(&local_ray);
        assert_eq!(ts.len(), 1);
        assert!(float::is_equal(ts[0], 1.0));
    }

    #[test]
    fn test_intersect_below() {
        let plane = Plane::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL
        );
        let local_ray= Ray::new(
            Tuple::point(0., -1., 0.),
            Tuple::vector(0., 1., 0.)
        );
        let ts = plane.intersect(&local_ray);
        assert_eq!(ts.len(), 1);
        assert!(float::is_equal(ts[0], 1.0));
    }
}