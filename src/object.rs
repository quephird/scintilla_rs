use crate::shape::Shape;
use crate::{material, ray, sphere, tuple};
use crate::intersection::Intersection;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::tuple::TupleMethods;

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

    pub fn normal_at(&self, world_point: tuple::Tuple) -> tuple::Tuple {
        let local_point = self.get_inverse_transform().multiply_tuple(world_point);
        let local_normal = match self {
            Object::Sphere(sphere) => sphere.normal_at(local_point),
        };
        let mut world_normal = self
            .get_inverse_transform()
            .transpose()
            .multiply_tuple(local_normal);
        world_normal[3] = 0.;
        world_normal.normalize()
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
