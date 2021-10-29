use crate::{matrix, tuple};
use crate::matrix::Matrix4Methods;
use crate::tuple::TupleMethods;

#[derive(Debug)]
pub struct Ray {
    pub origin: tuple::Tuple,
    pub direction: tuple::Tuple,
}

impl Ray {
    pub fn new(origin: tuple::Tuple, direction: tuple::Tuple) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn position_at(&self, t: f64) -> tuple::Tuple {
        self.origin.add(self.direction.multiply(t))
    }

    pub fn transform(&self, m: matrix::Matrix4) -> Ray {
        Ray {
            origin: m.multiply_tuple(self.origin),
            direction: m.multiply_tuple(self.direction),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transform;
    use crate::tuple::Tuple;
    use super::*;

    #[test]
    fn test_position_at() {
        let r = Ray::new([2., 3., 4., 1.],[1., 0., 0., 0.]);
        assert!(r.position_at(0.).is_equal(Tuple::point(2., 3., 4.)));
        assert!(r.position_at(1.).is_equal(Tuple::point(3., 3., 4.)));
        assert!(r.position_at(-1.).is_equal(Tuple::point(1., 3., 4.)));
        assert!(r.position_at(2.5).is_equal(Tuple::point(4.5, 3., 4.)));
    }

    #[test]
    fn test_transform_translation() {
        let r = Ray::new(
            Tuple::point(1., 2., 3.),
            Tuple::vector(0., 1., 0.)
        );
        let m = transform::translation(3., 4., 5.);
        let transformed_r = r.transform(m);
        assert!(transformed_r.origin.is_equal(Tuple::point(4., 6., 8.)));
        assert!(transformed_r.direction.is_equal(Tuple::vector(0., 1., 0.)));
    }

    #[test]
    fn test_transform_scaling() {
        let r = Ray::new(
            Tuple::point(1., 2., 3.),
            Tuple::vector(0., 1., 0.)
        );
        let m = transform::scaling(2., 3., 4.);
        let transformed_r = r.transform(m);
        assert!(transformed_r.origin.is_equal(Tuple::point(2., 6., 12.)));
        assert!(transformed_r.direction.is_equal(Tuple::vector(0., 3., 0.)));
    }
}
