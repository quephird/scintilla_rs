use std::f64::consts::PI;

use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::{material, matrix, pattern, transform};
use crate::material::Coloring::{SolidColor, SurfacePattern};
use crate::matrix::Matrix4Methods;
use crate::object::Object;
use crate::pattern::Pattern::StripedPattern;
use crate::pattern::Striped;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, TupleMethods};
use crate::world::World;

pub fn purple_sphere() -> World {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let material = Material {
        color: SolidColor(Color::new(1., 0.2, 1.)),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    };
    let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material,
        )
    );

    World::new(light, vec![sphere])
}

pub fn chapter_seven_scene() -> World {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let floor_material = Material {
        color: SolidColor(Color::new(1., 0.9, 0.9)),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.0,
        shininess: 200.0,
    };
    let floor = Object::Sphere(
        Sphere::new(
            transform::scaling(10., 0.01, 10.),
            floor_material.clone(),
        )
    );

    let transform_left_wall = transform::translation(0., 0., 5.)
        .multiply_matrix(transform::rotation_y(-PI/4.))
        .multiply_matrix(transform::rotation_x(PI/2.))
        .multiply_matrix(transform::scaling(10., 0.01, 10.));
    let left_wall = Object::Sphere(
        Sphere::new(
            transform_left_wall,
            floor_material.clone()
        )
    );

    let transform_right_wall = transform::translation(0., 0., 5.)
        .multiply_matrix(transform::rotation_y(PI/4.))
        .multiply_matrix(transform::rotation_x(PI/2.))
        .multiply_matrix(transform::scaling(10., 0.01, 10.));
    let right_wall = Object::Sphere(
        Sphere::new(
            transform_right_wall,
            floor_material.clone()
        )
    );

    let transform_middle = transform::translation(-0.5, 1., 0.5);
    let material_middle = Material {
        color: SolidColor(Color::new(0.1, 1., 0.5)),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0,
    };
    let middle_sphere = Object::Sphere(
        Sphere::new(
            transform_middle,
            material_middle
        )
    );

    let transform_right = transform::translation(1.5, 0.5, -0.5)
        .multiply_matrix(transform::scaling(0.5, 0.5, 0.5));
    let material_right = Material {
        color: SolidColor(Color::new(0.5, 1., 0.1)),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0,
    };
    let right_sphere = Object::Sphere(
        Sphere::new(
            transform_right,
            material_right
        )
    );

    let transform_left = transform::translation(-1.5, 0.33, -0.75)
        .multiply_matrix(transform::scaling(0.33, 0.33, 0.33));
    let material_left = Material {
        color: SolidColor(Color::new(1., 0.8, 0.1)),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0,
    };
    let left_sphere = Object::Sphere(
        Sphere::new(
            transform_left,
            material_left
        )
    );

    World::new(light, vec![floor, left_wall, right_wall, left_sphere, middle_sphere, right_sphere])
}

pub fn chapter_nine_scene() -> World {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let floor_material = Material {
        color: SolidColor(Color::new(1., 0.9, 0.9)),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.0,
        shininess: 200.0,
    };
    let floor = Object::Plane(
        Plane::new(
            matrix::IDENTITY,
            material::DEFAULT_MATERIAL,
        )
    );

    let transform_middle = transform::translation(-0.5, 1., 0.5);
    let material_middle = Material {
        color: SolidColor(Color::new(0.1, 1., 0.5)),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0,
    };
    let middle_sphere = Object::Sphere(
        Sphere::new(
            transform_middle,
            material_middle
        )
    );

    let transform_right = transform::translation(1.5, 0.5, -0.5)
        .multiply_matrix(transform::scaling(0.5, 0.5, 0.5));
    let material_right = Material {
        color: SolidColor(Color::new(0.5, 1., 0.1)),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0,
    };
    let right_sphere = Object::Sphere(
        Sphere::new(
            transform_right,
            material_right
        )
    );

    let transform_left = transform::translation(-1.5, 0.33, -0.75)
        .multiply_matrix(transform::scaling(0.33, 0.33, 0.33));
    let material_left = Material {
        color: SolidColor(Color::new(1., 0.8, 0.1)),
        ambient: 0.1,
        diffuse: 0.7,
        specular: 0.3,
        shininess: 200.0,
    };
    let left_sphere = Object::Sphere(
        Sphere::new(
            transform_left,
            material_left
        )
    );

    World::new(light, vec![floor, left_sphere, middle_sphere, right_sphere])
}
