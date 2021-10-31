use crate::{material, ray, tuple};

pub trait Shape {
    fn intersect(&self, ray: &ray::Ray) -> Vec<f64>;
    fn normal_at(&self, point: tuple::Tuple) -> tuple::Tuple;
    fn get_material(&self) -> material::Material;
}
