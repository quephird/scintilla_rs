use crate::shape::Shape;
use crate::{material, ray, sphere, tuple};
use crate::intersection::Intersection;

pub enum Object {
    Sphere(sphere::Sphere),
}

impl Shape for Object {
    fn intersect(&self, ray: &ray::Ray) -> Vec<Intersection> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
        }
    }

    fn normal_at(&self, point: tuple::Tuple) -> tuple::Tuple {
        match self {
            Object::Sphere(sphere) => sphere.normal_at(point),
        }
    }
    fn get_material(&self) -> material::Material {
        match self {
            Object::Sphere(sphere) => sphere.get_material(),
        }
    }
}
