use crate::{material, matrix, ray, tuple};
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Cone {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Cone {
    pub fn new(transform: Matrix4, material: Material) -> Cone {
        Cone {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
        }
    }
}

impl Shape for Cone {
    fn intersect(&self, local_ray: &ray::Ray) -> Vec<f64> {
        vec![1.]
    }

    fn normal_at(&self, local_point: tuple::Tuple) -> tuple::Tuple {
        Tuple::vector(0., 0., 1.)
    }
}
