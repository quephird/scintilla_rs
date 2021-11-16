use std::f64::consts::PI;

use crate::camera::Camera;
use crate::ppm::Saveable;
use crate::tuple::Tuple;
use crate::tuple::TupleMethods;

mod camera;
mod canvas;
mod color;
mod cube;
mod cylinder;
mod examples;
mod float;
mod intersection;
mod light;
mod material;
mod matrix;
mod object;
mod pattern;
mod plane;
mod ppm;
mod ray;
mod shape;
mod sphere;
mod transform;
mod tuple;
mod world;

fn main() {
    let world = examples::chapter_thirteen_scene();

    let from = Tuple::point(0., 3.5, -5.);
    let to = Tuple::point(0., 1., 0.);
    let up = Tuple::vector(0., 1., 0.);
    let view = transform::view(from, to, up);
    let camera = Camera::new(view, 800, 800, PI/2.);

    println!("Rendering scene...");
    let canvas = camera.render(world);

    println!("Saving file...");
    let result = canvas.save("test.ppm");
    match result {
        Ok(_) => println!("Done!!!"),
        Err(_) => println!("Whoops! Something went wrong"),
    }
}
