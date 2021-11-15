use crate::{material, matrix, ray, tuple};
use crate::material::Material;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::shape::Shape;
use crate::tuple::{Tuple, TupleMethods};

#[derive(Clone)]
pub struct Cylinder {
    pub transform: matrix::Matrix4,
    pub inverse_transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Cylinder {
    pub fn new(transform: Matrix4, material: Material) -> Cylinder {
        Cylinder {
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
            material: material,
        }
    }
}

impl Shape for Cylinder {
    fn intersect(&self, _local_ray: &ray::Ray) -> Vec<f64> {
        vec![]
    }

    fn normal_at(&self, _local_point: tuple::Tuple) -> tuple::Tuple {
        Tuple::vector(0., 0., 0.)
    }
}
