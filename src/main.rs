use std::f64::consts::PI;
use crate::camera::Camera;
use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::object::Object;
use crate::ppm::Saveable;
use crate::sphere::Sphere;
use crate::tuple::Tuple;
use crate::tuple::TupleMethods;
use crate::world::World;

mod camera;
mod canvas;
mod color;
mod float;
mod intersection;
mod light;
mod material;
mod matrix;
mod ppm;
mod ray;
mod shape;
mod sphere;
mod transform;
mod tuple;
mod object;
mod world;

fn main() {
    let light = Light::new(
        Tuple::point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    );

    let material = Material {
        color: Color::new(1., 0.2, 1.),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess: 200.0,
    };
    let sphere = Object::Sphere(Sphere::new(
        matrix::IDENTITY,
        material,
    ));

    let world = World::new(light, vec![sphere]);

    let from = Tuple::point(0., 1.5, -5.);
    let to = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let view = transform::view(from, to, up);
    let camera = Camera::new(view, 500, 500, PI/3.);

    println!("Rendering scene...");
    let canvas = camera.render(world);

    println!("Saving file...");
    let result = canvas.save("test.ppm");
    match result {
        Ok(_) => println!("Done!!!"),
        Err(_) => println!("Whoops! Something went wrong"),
    }
}
