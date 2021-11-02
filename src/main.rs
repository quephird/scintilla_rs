use std::f64::consts::PI;

use crate::camera::Camera;
use crate::ppm::Saveable;
use crate::tuple::Tuple;
use crate::tuple::TupleMethods;

mod camera;
mod canvas;
mod color;
mod examples;
mod float;
mod intersection;
mod light;
mod material;
mod matrix;
mod plane;
mod ppm;
mod ray;
mod shape;
mod sphere;
mod transform;
mod tuple;
mod object;
mod world;
mod pattern;

fn main() {
    let world = examples::chapter_ten_scene();

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
