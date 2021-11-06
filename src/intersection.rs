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

    pub fn prepare_computations(&self, ray: &Ray, all_intersections: Vec<Intersection>) -> Computations {
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
        let under_point = point.subtract(normal.multiply(float::EPSILON));
        let reflected = ray.direction.reflect(normal);

        let (n1, n2) = self.refractive_indices_for(all_intersections);

        Computations {
            t: self.t,
            point: point,
            eye: eye,
            normal: normal,
            reflected: reflected,
            is_inside: is_inside,
            object: self.object,
            over_point: over_point,
            under_point: under_point,
            n1: n1,
            n2: n2,
        }
    }

    pub fn refractive_indices_for(&self, all_intersections: Vec<Intersection>) -> (f64, f64) {
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        let mut containers: Vec<Intersection> = vec![];
        for intersection in all_intersections {
            let t = intersection.t;
            if t == self.t {
                n1 = match containers.last() {
                    Some(i) => i.object.get_material().refractive,
                    None => 1.0,
                };
            }
            match containers
                .iter()
                .position(|container| container.object.is_equal(intersection.object)) {
                Some(index) => {
                    containers.remove(index);
                    ()
                },
                None => {
                    containers.push(intersection)
                },
            };
            if t == self.t {
                n2 = match containers.last() {
                    Some(i) => i.object.get_material().refractive,
                    None => 1.0,
                };
                break;
            }
        }

        (n1, n2)
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
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
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
    use crate::{color, intersection, light, material, matrix, transform, tuple};
    use crate::sphere::Sphere;
    use crate::world::World;
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
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
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
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
        assert_eq!(computations.t, intersection.t);
        assert!(computations.point.is_equal(Tuple::point(0., 0., 1.)));
        assert!(computations.eye.is_equal(Tuple::vector(0., 0., -1.)));
        assert!(computations.normal.is_equal(Tuple::vector(0., 0., -1.)));
        assert_eq!(computations.is_inside, true);
    }

    #[test]
    fn test_prepare_computations_n1_n2() {
        let ta = transform::scaling(2., 2., 2.);
        let ma = material::DEFAULT_MATERIAL.with_refractive(1.5);
        let sphere_a = Object::Sphere(
            Sphere::new(ta, ma)
        );

        let tb = transform::translation(0., 0., -0.25);
        let mb = material::DEFAULT_MATERIAL.with_refractive(2.0);
        let sphere_b = Object::Sphere(
            Sphere::new(tb, mb)
        );

        let tc = transform::translation(0., 0., 0.25);
        let mc = material::DEFAULT_MATERIAL.with_refractive(2.5);
        let sphere_c = Object::Sphere(
            Sphere::new(tc, mc)
        );

        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );
        let world = World {
            light: light,
            objects: vec![sphere_a, sphere_b, sphere_c],
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -4.),
            Tuple::vector(0., 0., 1.),
        );

        let expected_values = [
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0),
        ];
        for i in 0..6 {
            let all_intersections = world.intersect(&ray);
            let hit = &all_intersections[i];
            let computations = hit.prepare_computations(&ray, all_intersections.clone());
            assert_eq!((computations.n1, computations.n2), expected_values[i]);
        }
    }
}
