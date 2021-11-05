use crate::float;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::Tuple;
use crate::tuple::TupleMethods;

#[derive(Clone)]
pub struct Intersection<'scene> {
    pub t: f64,
    pub object: &'scene Object,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: &Object) -> Intersection {
        Intersection {
            t: t,
            object: &object,
        }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let point = ray.position_at(self.t);
        let eye = ray.direction.negate();
        let mut normal = self.object.normal_at(point);

        let is_inside: bool;
        if normal.dot(eye) < 0. {
            is_inside = true;
            normal = normal.negate();
        } else {
            is_inside = false;
        }

        let over_point = point.add(normal.multiply(float::EPSILON));
        let reflected = ray.direction.reflect(normal);

        Computations {
            t: self.t,
            point: point,
            eye: eye,
            normal: normal,
            reflected: reflected,
            is_inside: is_inside,
            object: self.object,
            over_point: over_point
        }
    }
}

pub struct Computations<'scene> {
    pub t: f64,
    pub point: Tuple,
    pub eye: Tuple,
    pub normal: Tuple,
    pub reflected: Tuple,
    pub is_inside: bool,
    pub object: &'scene Object,
    pub over_point: Tuple,
}

pub fn hit<'a>(intersections: &'a mut Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());
    intersections
        .iter()
        .filter(|i| i.t >= 0.)
        .nth(0)
}

#[cfg(test)]
mod tests {
    use crate::{material, matrix};
    use crate::sphere::Sphere;
    use super::*;

    #[test]
    fn test_hit_all_positive_t() {
        let s = Object::Sphere(
            Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        ));
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let mut intersections = vec![i1.clone(), i2.clone()];
        let hit = hit(&mut intersections).unwrap();
        assert_eq!(hit.t, i1.t);
    }

    #[test]
    fn test_hit_mixture() {
        let s = Object::Sphere(Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        ));
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let mut intersections = vec![i1.clone(), i2.clone()];
        let hit = hit(&mut intersections).unwrap();
        assert_eq!(hit.t, i2.t);
    }

    #[test]
    fn test_hit_all_negative_t() {
        let s = Object::Sphere(Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        ));
        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let mut intersections = vec![i1.clone(), i2.clone()];
        let hit = hit(&mut intersections);
        assert!(hit.is_none());
    }

    #[test]
    fn test_hit_unsorted_intersection() {
        let s = Object::Sphere(Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        ));
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let mut intersections = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];
        let hit = hit(&mut intersections).unwrap();
        assert_eq!(hit.t, i4.t);
    }

    #[test]
    fn test_prepare_computations_outside() {
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let s = Object::Sphere(Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        ));
        let intersection = Intersection::new(4., &s);
        let computations = intersection.prepare_computations(&ray);
        assert_eq!(computations.t, intersection.t);
        assert!(computations.point.is_equal(Tuple::point(0., 0., -1.)));
        assert!(computations.eye.is_equal(Tuple::vector(0., 0., -1.)));
        assert!(computations.normal.is_equal(Tuple::vector(0., 0., -1.)));
        assert_eq!(computations.is_inside, false);
    }

    #[test]
    fn test_prepare_computations_inside() {
        let ray = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.)
        );
        let s = Object::Sphere(Sphere::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        ));
        let intersection = Intersection::new(1., &s);
        let computations = intersection.prepare_computations(&ray);
        assert_eq!(computations.t, intersection.t);
        assert!(computations.point.is_equal(Tuple::point(0., 0., 1.)));
        assert!(computations.eye.is_equal(Tuple::vector(0., 0., -1.)));
        assert!(computations.normal.is_equal(Tuple::vector(0., 0., -1.)));
        assert_eq!(computations.is_inside, true);
    }
}
