use crate::{float, material, matrix, ray, tuple};
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Cylinder {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Cylinder {
    pub fn new(transform: Matrix4, material: Material) -> Cylinder {
        Cylinder {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
        }
    }
}

impl Shape for Cylinder {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        let a = local_ray.direction[0]*local_ray.direction[0] +
            local_ray.direction[2]*local_ray.direction[2];

        if a < float::EPSILON {
            // Ray is parallel to the y axis
            vec![]
        } else {
            let b = 2. * local_ray.origin[0]*local_ray.direction[0] +
                2. * local_ray.origin[2]*local_ray.direction[2];
            let c = local_ray.origin[0]*local_ray.origin[0] +
                local_ray.origin[2]*local_ray.origin[2] - 1.;
            let discriminant = b*b - 4. * a * c;

            if discriminant < 0. {
                // Ray does not intersect the cylinder
                vec![]
            } else if discriminant == 0.0 {
                // Ray is tangent to cylinder
                vec![-b / (2. * a)]
            } else {
                // Ray _does_ intersect the cylinder twice
                let t1 = (-b - discriminant.sqrt()) / (2. * a);
                let t2 = (-b + discriminant.sqrt()) / (2. * a);
                vec![t1, t2]
            }
        }
    }

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        Tuple::vector(local_point[0], 0., local_point[2])
    }
}

#[cfg(test)]
mod tests {
    use crate::cylinder::{Cylinder, material};
    use crate::{float, matrix};
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::{Tuple, TupleMethods};

    #[test]
    fn test_intersect_miss() {
        let cylinder = Cylinder::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let test_cases = vec![
            (Tuple::point(1., 0., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 0., -5.), Tuple::vector(1., 1., 1.)),
        ];
        for (origin, direction) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let ts = cylinder.intersect(&ray);
            assert_eq!(ts.len(), 0);
        }
    }

    #[test]
    fn test_intersect_hits() {
        let cylinder = Cylinder::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let test_cases = vec![
            (Tuple::point(1., 0., -5.), Tuple::vector(0., 0., 1.), vec![5.]),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), vec![4., 6.]),
            (Tuple::point(0.5, 0., -5.), Tuple::vector(0.1, 1., 1.), vec![6.80798, 7.08872]),
        ];
        for (origin, direction, expected_ts) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let ts = cylinder.intersect(&ray);
            assert!(ts.iter().zip(expected_ts).all(|(&a, b)| float::is_equal(a, b)));
        }
    }

    #[test]
    fn test_normal_at() {
        let cylinder = Cylinder::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let test_cases = vec![
            (Tuple::point(1., 0., 0.), Tuple::vector(1., 0., 0.)),
            (Tuple::point(0., 5., -1.), Tuple::vector(0., 0., -1.)),
            (Tuple::point(0., -2., 1.), Tuple::vector(0., 0., 1.)),
            (Tuple::point(-1., 1., 0.), Tuple::vector(-1., 0., 0.)),
        ];

        for (point, expected_value) in test_cases {
            let normal = cylinder.normal_at(point);
            assert!(normal.is_equal(expected_value));
        }
    }
}
