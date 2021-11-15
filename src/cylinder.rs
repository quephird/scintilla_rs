use std::cmp::min;
use crate::{float, material, matrix, ray, tuple};
use crate::float::EPSILON;
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Cylinder {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
    pub minimum: f64,
    pub maximum: f64,
    pub is_closed: bool,
}

impl Cylinder {
    pub fn new_infinite(transform: Matrix4, material: Material) -> Cylinder {
        Cylinder {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
            minimum: -f64::INFINITY,
            maximum: f64::INFINITY,
            is_closed: false,
        }
    }

    pub fn new_truncated(transform: Matrix4, material: Material, minimum: f64, maximum: f64) -> Cylinder {
        Cylinder {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
            minimum: minimum,
            maximum: maximum,
            is_closed: false,
        }
    }

    pub fn new_capped(transform: Matrix4, material: Material, minimum: f64, maximum: f64) -> Cylinder {
        Cylinder {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
            minimum: minimum,
            maximum: maximum,
            is_closed: true,
        }
    }

    // This is a helper function to reduce code duplication,
    // checks to see if the intersection at `t` is within a radius
    // of 1 (the radius of your cylinders) from the y axis.
    fn check_cap(&self, local_ray: &ray::Ray, t: f64) -> bool {
        let x = local_ray.origin[0] + t * local_ray.direction[0];
        let z = local_ray.origin[2] + t * local_ray.direction[2];
        (x*x + z*z) <= 1.
    }

    fn intersect_caps(&self, local_ray: &ray::Ray) -> Vec<f64> {
        // Caps only matter if the cylinder is closed, and might possibly be
        // intersected by the ray.
        if !self.is_closed || local_ray.direction[1].abs() < float::EPSILON {
            vec![]
        } else {
            let mut ts = vec![];

            // Check for an intersection with the lower end cap by intersecting
            // the ray with the plane at cylinder minimum.
            let t1 = (self.minimum - local_ray.origin[1]) / local_ray.direction[1];
            if self.check_cap(local_ray, t1) {
                ts.push(t1);
            }

            // Now check for an intersection with the upper end cap by intersecting
            // the ray with the plane at cylinder maximum.
            let t2 = (self.maximum - local_ray.origin[1]) / local_ray.direction[1];
            if self.check_cap(local_ray, t2) {
                ts.push(t2);
            }

            ts
        }
    }

    fn intersect_walls(&self, local_ray: &ray::Ray) -> Vec<f64> {
        let a = local_ray.direction[0]*local_ray.direction[0] +
            local_ray.direction[2]*local_ray.direction[2];

        if a.abs() < float::EPSILON {
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
                // Ray is potentially tangent to cylinder
                let t = -b / (2. * a);
                let y = local_ray.origin[1] + local_ray.direction[1]*t;
                if y > self.minimum && y < self.maximum {
                    vec![t]
                } else {
                    vec![]
                }
            } else {
                // Ray _does_ potentially intersect the cylinder twice
                let t1 = (-b - discriminant.sqrt()) / (2. * a);
                let t2 = (-b + discriminant.sqrt()) / (2. * a);

                let mut ts = vec![];
                let y1 = local_ray.origin[1] + local_ray.direction[1]*t1;
                if y1 > self.minimum && y1 < self.maximum {
                    ts.push(t1);
                }

                let y2 = local_ray.origin[1] + local_ray.direction[1]*t2;
                if y2 > self.minimum && y2 < self.maximum {
                    ts.push(t2);
                }

                ts
            }
        }
    }
}

impl Shape for Cylinder {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        let mut wall_ts = self.intersect_walls(local_ray);
        let mut caps_ts = self.intersect_caps(local_ray);

        wall_ts.append(&mut caps_ts);
        wall_ts
    }

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        let distance = local_point[0] * local_point[0] +
            local_point[2] * local_point[2];

        if distance < 1. && local_point[1] >= self.maximum - EPSILON {
            Tuple::vector(0., 1., 0.)
        } else if distance < 1. && local_point[1] <= self.minimum + EPSILON {
            Tuple::vector(0., -1., 0.)
        } else {
            Tuple::vector(local_point[0], 0., local_point[2])
        }
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
    fn test_intersect_miss_infinite() {
        let cylinder = Cylinder::new_infinite(
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
    fn test_intersect_hits_infinite() {
        let cylinder = Cylinder::new_infinite(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let test_cases = vec![
            // (Tuple::point(1., 0., -5.), Tuple::vector(0., 0., 1.), vec![5.]),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), vec![4., 6.]),
            // (Tuple::point(0.5, 0., -5.), Tuple::vector(0.1, 1., 1.), vec![6.80798, 7.08872]),
        ];
        for (origin, direction, expected_ts) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let ts = cylinder.intersect(&ray);
            assert!(ts.iter().zip(expected_ts).all(|(&a, b)| float::is_equal(a, b)));
        }
    }

    #[test]
    fn test_intersect_hits_truncated() {
        let cylinder = Cylinder::new_truncated(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
            1., 2.,
        );

        let test_cases = vec![
            (Tuple::point(0., 1.5, 0.), Tuple::vector(0.1, 1., 0.), 0),
            (Tuple::point(0., 3., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.), 0),
            (Tuple::point(0., 1.5, -2.), Tuple::vector(0., 0., 1.), 2),
        ];
        for (origin, direction, expected_count) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let ts = cylinder.intersect(&ray);
            assert_eq!(ts.len(), expected_count);
        }
    }

    #[test]
    fn test_intersect_hits_capped() {
        let cylinder = Cylinder::new_capped(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
            1., 2.,
        );

        let test_cases = vec![
            (Tuple::point(0., 3., 0.), Tuple::vector(0., -1., 0.), 2),
            (Tuple::point(0., 3., -2.), Tuple::vector(0., -1., 2.), 2),
            (Tuple::point(0., 4., -2.), Tuple::vector(0., -1., 1.), 2),
            (Tuple::point(0., 0., -2.), Tuple::vector(0., 1., 2.), 2),
            (Tuple::point(0., -1., -2.), Tuple::vector(0., 1., 1.), 2),
        ];
        for (origin, direction, expected_count) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let ts = cylinder.intersect(&ray);
            assert_eq!(ts.len(), expected_count);
        }
    }

    #[test]
    fn test_normal_at_infinite() {
        let cylinder = Cylinder::new_infinite(
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

    #[test]
    fn test_normal_at_capped() {
        let cylinder = Cylinder::new_capped(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
            1., 2.,
        );

        let test_cases = vec![
            (Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0.5, 1., 0.), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0., 1., 0.5), Tuple::vector(0., -1., 0.)),
            (Tuple::point(0., 2., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0.5, 2., 0.), Tuple::vector(0., 1., 0.)),
            (Tuple::point(0., 2., 0.5), Tuple::vector(0., 1., 0.)),
        ];

        for (point, expected_value) in test_cases {
            let normal = cylinder.normal_at(point);
            assert!(normal.is_equal(expected_value));
        }
    }
}
