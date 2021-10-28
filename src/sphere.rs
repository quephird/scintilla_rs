use crate::float;
use crate::intersection::Intersection;
use crate::material;
use crate::matrix;
use crate::ray;
use crate::shape::Shape;
use crate::tuple;

pub struct Sphere {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: matrix::IDENTITY,
            inverse_transform: matrix::IDENTITY,
            material: material::DEFAULT_MATERIAL,
        }
    }

    pub fn set_transform(&mut self, m: matrix::Matrix4) {
        self.transform = m;
        self.inverse_transform = matrix::inverse_4x4(m).unwrap();
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &ray::Ray) -> Vec<Intersection> {
        let inverse_transform = matrix::inverse_4x4(self.transform).unwrap();
        let transformed_ray = ray.transform(inverse_transform);
        let sphere_to_ray = tuple::subtract(transformed_ray.origin, [0., 0., 0., 1.]);
        let a = tuple::dot(transformed_ray.direction, transformed_ray.direction);
        let b = 2. * tuple::dot(transformed_ray.direction, sphere_to_ray);
        let c = tuple::dot(sphere_to_ray, sphere_to_ray) - 1.;
        let discriminant = b*b - 4.*a*c;

        if discriminant < 0. {
            vec![]
        } else if discriminant == 0. {
            let i = Intersection::new(-b/2./a, self);
            vec![i]
        } else {
            let i1 = Intersection::new((-b - discriminant.sqrt())/2./a, self);
            let i2 = Intersection::new((-b + discriminant.sqrt())/2./a, self);
            vec![i1, i2]
        }
    }

    fn normal_at(&self, world_point: tuple::Tuple) -> tuple::Tuple {
        let object_point = matrix::multiply_by_tuple(self.inverse_transform, world_point);
        let object_normal = tuple::subtract(object_point, tuple::point(0.,0.,0.));
        let mut world_normal = matrix::multiply_by_tuple(matrix::transpose(self.inverse_transform), object_normal);
        world_normal[3] = 0.;
        tuple::normalize(world_normal)
    }

    fn get_material(&self) -> material::Material {
        self.material.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::matrix::multiply_by_matrix;
    use crate::transform;
    use super::*;

    #[test]
    fn test_intersect_miss() {
        let ray = ray::Ray::new([0., 2., -5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersect_tangent() {
        let ray = ray::Ray::new([0., 1., -5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(float::is_equal(intersections[0].t, 5.), true);
    }

    #[test]
    fn test_intersect_inside() {
        let ray = ray::Ray::new([0., 0., 0., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0].t, -1.), true);
        assert_eq!(float::is_equal(intersections[1].t, 1.), true);
    }

    #[test]
    fn test_intersect_behind() {
        let ray = ray::Ray::new([0., 0., 5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0].t, -6.), true);
        assert_eq!(float::is_equal(intersections[1].t, -4.), true);
    }

    #[test]
    fn test_intersect_scaled() {
        let ray = ray::Ray::new([0., 0., -5., 1.], [0., 0., 1., 0.]);
        let mut sphere = Sphere::new();
        let transform = transform::scaling(2., 2., 2.);
        sphere.set_transform(transform);

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0].t, 3.), true);
        assert_eq!(float::is_equal(intersections[1].t, 7.), true);
    }

    #[test]
    fn test_intersect_translated() {
        let ray = ray::Ray::new([0., 0., -5., 1.], [0., 0., 1., 0.]);
        let mut sphere = Sphere::new();
        let transform = transform::translation(5., 0., 0.);
        sphere.set_transform(transform);

        let intersections = sphere.intersect(&ray);
        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_normal_at_point_on_x_axis() {
        let s = Sphere::new();
        let normal = s.normal_at(tuple::point(1., 0., 0.));
        assert!(tuple::is_equal(normal, tuple::vector(1., 0., 0.)));
    }

    #[test]
    fn test_normal_at_point_on_y_axis() {
        let s = Sphere::new();
        let normal = s.normal_at(tuple::point(0., 1., 0.));
        assert!(tuple::is_equal(normal, tuple::vector(0., 1., 0.)));
    }

    #[test]
    fn test_normal_at_point_on_z_axis() {
        let s = Sphere::new();
        let normal = s.normal_at(tuple::point(0., 0., 1.));
        assert!(tuple::is_equal(normal, tuple::vector(0., 0., 1.)));
    }

    #[test]
    fn test_normal_at_nonaxial_point() {
        let s = Sphere::new();
        let normal = s.normal_at(tuple::point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));
        assert!(tuple::is_equal(normal, tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)));
    }

    #[test]
    fn test_normal_at_for_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(transform::translation(0.,1.,0.));
        let normal = s.normal_at(tuple::point(0.,1.70711,-0.70711));
        let expected_value = tuple::vector(0.,0.70711,-0.70711);
        assert!(tuple::is_equal(normal, expected_value));
    }

    #[test]
    fn test_normal_at_for_transformed_sphere() {
        let mut sphere = Sphere::new();
        let s = transform::scaling(1., 0.5, 1.);
        let rz = transform::rotation_z(PI/5.);
        let transform = multiply_by_matrix(s, rz);
        sphere.set_transform(transform);
        let normal = sphere.normal_at(tuple::point(0.,0.70711,-0.70711));
        let expected_value = tuple::vector(0., 0.97014, -0.24254);
        assert!(tuple::is_equal(normal, expected_value));
    }
}