use crate::intersection::Intersection;
use crate::ray;

pub trait Shape {
    fn intersect(&self, ray: ray::Ray) -> Vec<Intersection>;
}
