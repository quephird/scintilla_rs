use std::f64::consts::PI;

use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::{color, material, matrix, pattern, transform};
use crate::cube::Cube;
use crate::material::Coloring::{SolidColor, SurfacePattern};
use crate::matrix::Matrix4Methods;
use crate::object::Object;
use crate::pattern::Pattern::{Checker2DPattern, Checker3DPattern, GradientPattern, Ring3DPattern, RingPattern};
use crate::pattern::Pattern::StripedPattern;
use crate::pattern::{Checker2D, Checker3D, Gradient, Ring, Ring3D, Striped};
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::transform::rotation_y;
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
    };
    let sphere = Object::Sphere(
            Sphere::new(
                transform::translation(-2., 0., 0.),
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
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
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
    };
    let left_sphere = Object::Sphere(
        Sphere::new(
            transform_left,
            material_left
        )
    );

    World::new(light, vec![floor, left_sphere, middle_sphere, right_sphere])
}

pub fn chapter_ten_scene() -> World {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let striped = SurfacePattern(
        StripedPattern(
            Striped::new(
                Color::new(1.0, 0.5, 0.0),
                Color::new(0.8, 0.2, 1.0),
                transform::scaling(0.25, 1.0, 1.0),
            )
        )
    );
    let material = Material {
        color: striped,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
    };
    let striped_sphere = Object::Sphere(
        Sphere::new(
            transform::translation(-2.5, 1.0, 0.0)
                .multiply_matrix(transform::rotation_z(PI/4.)),
            material,
        )
    );

    let gradient = SurfacePattern(
        GradientPattern(
            Gradient::new(
                Color::new(0.0, 0.8, 0.1),
                Color::new(0.8, 0.2, 0.2),
                transform::scaling(2., 1.0, 1.0)
                    .multiply_matrix(transform::translation(-0.5, 0., 0.)),
            )
        )
    );
    let material = Material {
        color: gradient,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
    };
    let gradient_sphere = Object::Sphere(
        Sphere::new(
            transform::translation(0.0, 1.0, 0.0),
            material,
        )
    );

    let checkered = SurfacePattern(
        Checker3DPattern(
            Checker3D::new(
                Color::new(0.0, 0.2, 0.8),
                Color::new(0.8, 0.9, 0.1),
                transform::scaling(0.4, 0.4, 0.4)
                    .multiply_matrix(rotation_y(PI/2.)),
            )
        )
    );
    let material = Material {
        color: checkered,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
    };
    let checkered_sphere = Object::Sphere(
        Sphere::new(
            transform::translation(2.5, 1.0, 0.0),
            material,
        )
    );

    let rings = SurfacePattern(
        RingPattern(
            Ring::new(
                Color::new(1., 0.9, 0.9),
                Color::new(0.6, 0.6, 0.6),
                matrix::IDENTITY,
            )
        )
    );
    let floor_material = Material {
        color: rings,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.0,
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.0,
        refractive: 1.0,
    };
    let floor = Object::Plane(
        Plane::new(
            matrix::IDENTITY,
            floor_material,
        )
    );

    World::new(light, vec![gradient_sphere, striped_sphere, checkered_sphere, floor])
}

pub fn chapter_eleven_scene() -> World {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let glass = Material {
        color: SolidColor(Color::new(0.2, 0.0, 0.2)),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.0,
        transparency: 0.9,
        refractive: 1.52,
    };
    let glass_ball = Object::Sphere(
        Sphere::new(
            transform::translation(0., 1., 0.),
            glass,
        )
    );

    let green_metal = Material {
        color: SolidColor(Color::new(0.0, 0.4, 0.0)),
        ambient: 0.5,
        diffuse: 0.1,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.9,
        transparency: 0.0,
        refractive: 0.0,
    };
    let green_metallic_ball = Object::Sphere(
        Sphere::new(
            transform::translation(2.5, 1., 0.),
            green_metal,
        )
    );

    let red_metal = Material {
        color: SolidColor(Color::new(0.4, 0.0, 0.0)),
        ambient: 0.3,
        diffuse: 0.3,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.9,
        transparency: 0.0,
        refractive: 0.0,
    };
    let red_metallic_ball = Object::Sphere(
        Sphere::new(
            transform::translation(-2.5, 1., 0.),
            red_metal,
        )
    );

    let orange = Material {
        color: SolidColor(Color::new(0.8, 0.4, 0.0)),
        ambient: 0.2,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.1,
        transparency: 0.0,
        refractive: 0.0,
    };
    let orange_ball = Object::Sphere(
        Sphere::new(
            transform::translation(-1.5, 1., 2.5),
            orange,
        )
    );

    let yellow = Material {
        color: SolidColor(Color::new(0.8, 0.8, 0.0)),
        ambient: 0.2,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.1,
        transparency: 0.0,
        refractive: 0.0,
    };
    let yellow_ball = Object::Sphere(
        Sphere::new(
            transform::translation(1.5, 1., 2.5),
            yellow,
        )
    );

    let checkered = SurfacePattern(
        Checker2DPattern(
            Checker2D::new(
                color::WHITE,
                color::BLACK,
                transform::rotation_y(PI/4.),
            )
        )
    );
    let floor_material = Material {
        color: checkered,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.4,
        transparency: 0.0,
        refractive: 1.0,
    };
    let floor = Object::Plane(
        Plane::new(
            matrix::IDENTITY,
            floor_material,
        )
    );

    World::new(light, vec![
        glass_ball,
        red_metallic_ball,
        green_metallic_ball,
        orange_ball,
        yellow_ball,
        floor,
    ])
}

pub fn chapter_twelve_scene() -> World {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let transform = transform::translation(0., 1., 0.)
        .multiply_matrix(transform::rotation_y(PI/4.));
    let ringed = SurfacePattern(
        Ring3DPattern(
            Ring3D::new(
                Color::new(1., 0., 0.),
                Color::new(0., 1., 0.),
                transform::scaling(0.1, 0.1, 0.1),
            )
        )
    );
    let material = Material {
        color: ringed,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.1,
        transparency: 0.0,
        refractive: 1.0,
    };
    let cube = Object::Cube(
        Cube::new(
            transform,
            material,
        )
    );

    let checkered = SurfacePattern(
        Checker2DPattern(
            Checker2D::new(
                color::WHITE,
                color::BLACK,
                transform::rotation_y(PI/3.),
            )
        )
    );
    let floor_material = Material {
        color: checkered,
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
        reflective: 0.4,
        transparency: 0.0,
        refractive: 1.0,
    };
    let floor = Object::Plane(
        Plane::new(
            matrix::IDENTITY,
            floor_material,
        )
    );

    World::new(light, vec![cube, floor])
}
