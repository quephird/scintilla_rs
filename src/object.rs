use crate::shape::Shape;
use crate::{cube, cylinder, material, plane, ray, sphere, tuple};
use crate::intersection::Intersection;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::tuple::TupleMethods;

#[derive(Clone)]
pub enum Object {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
    Cube(cube::Cube),
    Cylinder(cylinder::Cylinder),
}

impl Object {
    pub fn intersect(&self, world_ray: &ray::Ray) -> Vec<Intersection> {
        let local_ray = world_ray.transform(self.get_inverse_transform());
        let ts = match self {
            Object::Sphere(sphere) => sphere.intersect(&local_ray),
            Object::Plane(plane) => plane.intersect(&local_ray),
            Object::Cube(cube) => cube.intersect(&local_ray),
            Object::Cylinder(cylinder) => cylinder.intersect(&local_ray),
        };
        ts.iter()
            .map(|&t| Intersection::new(t, self))
            .collect()
    }

    pub fn as_shape(&self) -> &dyn Shape {
        match self {
            Object::Sphere(sphere) => sphere,
            Object::Plane(plane) => plane,
            Object::Cube(cube) => cube,
            Object::Cylinder(cylinder) => cylinder,
        }
    }

    pub fn normal_at(&self, world_point: tuple::Tuple) -> tuple::Tuple {
        let local_point = self.get_inverse_transform().multiply_tuple(world_point);
        let local_normal = match self {
            Object::Sphere(sphere) => sphere.normal_at(local_point),
            Object::Plane(plane) => plane.normal_at(local_point),
            Object::Cube(cube) => cube.normal_at(local_point),
            Object::Cylinder(cylinder) => cylinder.normal_at(local_point),
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
            Object::Plane(plane) => plane.inverse_transform,
            Object::Cube(cube) => cube.inverse_transform,
            Object::Cylinder(cylinder) => cylinder.inverse_transform,
        }
    }

    pub fn get_material(&self) -> &material::Material {
        match self {
            Object::Sphere(sphere) => &sphere.material,
            Object::Plane(plane) => &plane.material,
            Object::Cube(cube) => &cube.material,
            Object::Cylinder(cylinder) => &cylinder.material,
        }
    }

    // TODO: This is a hack; ideally we really need an object ID in each shape
    pub fn is_equal(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Sphere(s1), Object::Sphere(s2)) =>
                s1.transform.is_equal(s2.transform),
            (Object::Plane(p1), Object::Plane(p2)) =>
                p1.transform.is_equal(p2.transform),
            (Object::Cube(c1), Object::Cube(c2)) =>
                c1.transform.is_equal(c2.transform),
            (Object::Cylinder(c1), Object::Cylinder(c2)) =>
                c1.transform.is_equal(c2.transform),
            _ => false,
        }
    }
}
