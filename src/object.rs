use crate::shape::Shape;
use crate::{material, ray, sphere, tuple};
use crate::intersection::Intersection;

pub enum Object {
    Sphere(sphere::Sphere),
}

impl Object {
    pub fn intersect(&self, ray: &ray::Ray) -> Vec<Intersection> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray)
                .iter()
                .map(|&t| Intersection::new(t, self))
                .collect()
        }
    }

    pub fn as_shape(&self) -> &dyn Shape {
        match self {
            Object::Sphere(sphere) => sphere,
        }
    }

    pub fn normal_at(&self, point: tuple::Tuple) -> tuple::Tuple {
        match self {
            Object::Sphere(sphere) => sphere.normal_at(point),
        }
    }
    pub fn get_material(&self) -> material::Material {
        match self {
            Object::Sphere(sphere) => sphere.get_material(),
        }
    }
}
