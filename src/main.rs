use crate::color::Color;
use crate::light::Light;
use crate::ppm::Saveable;
use crate::shape::Shape;
use crate::tuple::Tuple;
use crate::tuple::TupleMethods;

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

fn main() {
    let ray_origin = [0., 0., -5., 1.];
    let wall_z = 10.;
    let wall_size = 7.;
    let half = wall_size / 2.;
    let canvas_pixels = 500;
    let pixel_size = wall_size / canvas_pixels as f64;

    let mut canvas = canvas::Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = sphere::Sphere::new();
    shape.material.color = Color::new(1., 0.2, 1.);

    let light_position = Tuple::point(-10., 10., -10.);
    let light_color = Color::new(1., 1., 1.);
    let light = Light::new(light_position, light_color);

    // For each row of pixels in the canvas
    println!("Rendering scene...");
    for y in 0..canvas.height {
        // Compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f64;
        // For each pixel in the row
        for x in 0..canvas.width {
            // Compute the world x coordinate (left = -half, right = half)
            let world_x = -half + pixel_size * x as f64;
            // Describe the point on the wall that the ray will target
            let position = [world_x, world_y, wall_z, 1.0];
            let direction = position.subtract(ray_origin).normalize();

            let ray = ray::Ray::new(ray_origin, direction);
            let mut intersections = shape.intersect(&ray);
            match intersection::hit(&mut intersections) {
                Some(hit) => {
                    let point = ray.position_at(hit.t);
                    let normal = hit.object.normal_at(point);
                    let eye = ray.direction.negate();
                    let color = hit.object.get_material().lighting(&light, point, eye, normal);
                    canvas.set_pixel(x, y, color);
                },
                None => ()
            }
        }
    }

    println!("Saving file...");
    canvas.save("test.ppm");
    println!("Done!!!");
}
