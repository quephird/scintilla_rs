use crate::shape::Shape;
use crate::{material, ray, sphere, tuple};
use crate::intersection::Intersection;
use crate::matrix::Matrix4;

pub enum Object {
    Sphere(sphere::Sphere),
}

impl Object {
    pub fn intersect(&self, world_ray: &ray::Ray) -> Vec<Intersection> {
        let local_ray = world_ray.transform(self.get_inverse_transform());
        match self {
            Object::Sphere(sphere) => sphere.intersect(&local_ray)
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

    pub fn get_inverse_transform(&self) -> Matrix4 {
        match self {
            Object::Sphere(sphere) => sphere.inverse_transform,
        }
    }

    pub fn get_material(&self) -> material::Material {
        match self {
            Object::Sphere(sphere) => sphere.material,
        }
    }
}
