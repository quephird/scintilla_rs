use crate::{float, material, matrix, ray, tuple};
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Cone {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
    pub minimum: f64,
    pub maximum: f64,
}

impl Cone {
    pub fn new_infinite(transform: Matrix4, material: Material) -> Cone {
        Cone {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
            minimum: -f64::INFINITY,
            maximum: f64::INFINITY,
        }
    }
}

impl Shape for Cone {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        let a = local_ray.direction[0]*local_ray.direction[0] -
            local_ray.direction[1]*local_ray.direction[1] +
            local_ray.direction[2]*local_ray.direction[2];
        let b = 2. * local_ray.origin[0]*local_ray.direction[0] -
            2. * local_ray.origin[1]*local_ray.direction[1] +
            2. * local_ray.origin[2]*local_ray.direction[2];
        let c = local_ray.origin[0]*local_ray.origin[0] -
            local_ray.origin[1]*local_ray.origin[1] +
            local_ray.origin[2]*local_ray.origin[2];

        if a.abs() < float::EPSILON && b.abs() < float::EPSILON {
            // Ray is parallel to cones but intersects neither
            vec![]
        } else if a.abs() < float::EPSILON && b.abs() > float::EPSILON {
            // Ray is parallel to cones but intersects one of them
            vec![-c/2./b]
        } else {
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

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        Tuple::vector(0., 0., 1.)
    }
}

#[cfg(test)]
mod tests {
    use crate::cone::Cone;
    use crate::{float, material, matrix};
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::{Tuple, TupleMethods};

    #[test]
    fn test_intersect_infinite_hits_twice() {
        let cone = Cone::new_infinite(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let test_cases = vec![
            (Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.), vec![5.]),
            (Tuple::point(0., 0., -5.), Tuple::vector(1., 1., 1.), vec![8.66025]),
            (Tuple::point(1., 1., -5.), Tuple::vector(-0.5, -1., 1.), vec![4.55006, 49.44994]),
        ];
        for (origin, direction, expected_ts) in test_cases {
            let ray = Ray::new(origin, direction.normalize());
            let ts = cone.intersect(&ray);
            assert!(ts.iter().zip(expected_ts).all(|(&a, b)| float::is_equal(a, b)));
        }
    }

    #[test]
    fn test_intersect_infinite_hits_once() {
        let cone = Cone::new_infinite(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        );

        let ray = Ray::new(
            Tuple::point(0., 0., -1.),
            Tuple::vector(0., 1., 1.).normalize(),
        );
        let ts = cone.intersect(&ray);
        assert_eq!(ts.len(), 1);
        assert!(float::is_equal(ts[0], 0.35355));
    }
}
