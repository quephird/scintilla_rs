use crate::float;
use crate::material;
use crate::material::Material;
use crate::matrix;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::ray;
use crate::shape::Shape;
use crate::tuple;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Sphere {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Sphere {
    pub fn new(transform: Matrix4, material: Material) -> Sphere {
        Sphere {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        let sphere_to_ray = local_ray.origin.subtract([0., 0., 0., 1.]);
        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2. * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
        let discriminant = b*b - 4.*a*c;

        if discriminant < 0. {
            vec![]
        } else if discriminant == 0. {
            vec![-b/2./a]
        } else {
            vec![(-b - discriminant.sqrt())/2./a, (-b + discriminant.sqrt())/2./a,]
        }
    }

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        local_point.subtract(Tuple::point(0.,0.,0.))
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::matrix::Matrix4Methods;
    use crate::transform;
    use crate::tuple::Tuple;
    use super::*;

    #[test]
    fn test_intersect_miss() {
        let ray = ray::Ray::new([0., 2., -5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersect_tangent() {
        let ray = ray::Ray::new([0., 1., -5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(float::is_equal(intersections[0], 5.), true);
    }

    #[test]
    fn test_intersect_inside() {
        let ray = ray::Ray::new([0., 0., 0., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0], -1.), true);
        assert_eq!(float::is_equal(intersections[1], 1.), true);
    }

    #[test]
    fn test_intersect_behind() {
        let ray = ray::Ray::new([0., 0., 5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0], -6.), true);
        assert_eq!(float::is_equal(intersections[1], -4.), true);
    }

    #[test]
    fn test_intersect_scaled() {
        let sphere = Sphere::new(
            transform::scaling(2., 2., 2.),
            material::DEFAULT_MATERIAL,
        );
        let world_ray = ray::Ray::new([0., 0., -5., 1.], [0., 0., 1., 0.]);
        let local_ray = world_ray.transform(sphere.inverse_transform);

        let intersections = sphere.intersect(&local_ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0], 3.), true);
        assert_eq!(float::is_equal(intersections[1], 7.), true);
    }

    #[test]
    fn test_intersect_translated() {
        let sphere = Sphere::new(
            transform::translation(5., 0., 0.),
            material::DEFAULT_MATERIAL,
        );
        let world_ray = ray::Ray::new([0., 0., -5., 1.], [0., 0., 1., 0.]);
        let local_ray = world_ray.transform(sphere.inverse_transform);

        let intersections = sphere.intersect(&local_ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_normal_at_point_on_x_axis() {
        let s = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let normal = s.normal_at(Tuple::point(1., 0., 0.));
        assert!(normal.is_equal(Tuple::vector(1., 0., 0.)));
    }

    #[test]
    fn test_normal_at_point_on_y_axis() {
        let s = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let normal = s.normal_at(Tuple::point(0., 1., 0.));
        assert!(normal.is_equal(Tuple::vector(0., 1., 0.)));
    }

    #[test]
    fn test_normal_at_point_on_z_axis() {
        let s = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let normal = s.normal_at(Tuple::point(0., 0., 1.));
        assert!(normal.is_equal(Tuple::vector(0., 0., 1.)));
    }

    #[test]
    fn test_normal_at_nonaxial_point() {
        let s = Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );
        let normal = s.normal_at(Tuple::point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));
        assert!(normal.is_equal(Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)));
    }

    #[test]
    fn test_normal_at_for_translated_sphere() {
        let sphere = Sphere::new(
            transform::translation(0.,1.,0.),
            material::DEFAULT_MATERIAL,
        );
        let world_point = Tuple::point(0.,1.70711,-0.70711);
        let local_point = sphere.inverse_transform.multiply_tuple(world_point);
        let local_normal = sphere.normal_at(local_point);
        let expected_value = Tuple::vector(0.,0.70711,-0.70711);
        assert!(local_normal.is_equal(expected_value));
    }

    #[test]
    fn test_normal_at_for_transformed_sphere() {
        let s = transform::scaling(1., 0.5, 1.);
        let rz = transform::rotation_z(PI/5.);
        let transform = s.multiply_matrix(rz);
        let sphere = Sphere::new(
            transform,
            material::DEFAULT_MATERIAL,
        );
        let world_point = Tuple::point(0.,0.70711,-0.70711);
        let local_point = sphere.inverse_transform.multiply_tuple(world_point);
        let local_normal = sphere.normal_at(local_point);
        let expected_value = Tuple::vector(0.83126, 1.14413, -0.70711);
        assert!(local_normal.is_equal(expected_value));
    }
}