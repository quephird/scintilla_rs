use crate::{float, matrix};
use crate::intersection::Intersection;
use crate::ray;
use crate::shape::Shape;
use crate::tuple;

pub struct Sphere {
    pub transform: matrix::Matrix4,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: matrix::IDENTITY
        }
    }

    pub fn set_transform(&mut self, m: matrix::Matrix4) {
        self.transform = m;
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: ray::Ray) -> Vec<Intersection> {
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
}

#[cfg(test)]
mod tests {
    use crate::transform;
    use super::*;

    #[test]
    fn test_intersect_miss() {
        let ray = ray::Ray::new([0., 2., -5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersect_tangent() {
        let ray = ray::Ray::new([0., 1., -5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(float::is_equal(intersections[0].t, 5.), true);
    }

    #[test]
    fn test_intersect_inside() {
        let ray = ray::Ray::new([0., 0., 0., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(float::is_equal(intersections[0].t, -1.), true);
        assert_eq!(float::is_equal(intersections[1].t, 1.), true);
    }

    #[test]
    fn test_intersect_behind() {
        let ray = ray::Ray::new([0., 0., 5., 1.], [0., 0., 1., 0.]);
        let sphere = Sphere::new();
        let intersections = sphere.intersect(ray);

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

        let intersections = sphere.intersect(ray);
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

        let intersections =  sphere.intersect(ray);
        assert_eq!(intersections.len(), 0);
    }
}
