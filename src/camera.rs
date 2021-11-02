use crate::canvas::Canvas;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::ray::Ray;
use crate::tuple::{Tuple, TupleMethods};
use crate::world::World;

pub struct Camera {
    pub view: Matrix4,
    pub horizontal_size: usize,
    pub vertical_size: usize,
    pub field_of_view: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(view: Matrix4, horizontal_size: usize, vertical_size: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect = horizontal_size as f64 / vertical_size as f64;
        let half_width: f64;
        let half_height: f64;
        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.) / horizontal_size as f64;

        Camera {
            view: view,
            horizontal_size: horizontal_size,
            vertical_size: vertical_size,
            field_of_view: field_of_view,
            half_width: half_width,
            half_height: half_height,
            pixel_size: pixel_size,
        }
    }

    pub fn ray_at(&self, pixel_x: usize, pixel_y: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let offset_x = (pixel_x as f64 + 0.5) * self.pixel_size;
        let offset_y = (pixel_y as f64 + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space.
        // (Remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        // Using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (Remember that the canvas is at z=-1)
        let pixel = self.view.inverse().unwrap().multiply_tuple(Tuple::point(world_x, world_y, -1.));
        let origin = self.view.inverse().unwrap().multiply_tuple(Tuple::point(0., 0., 0.));
        let direction = pixel.subtract(origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = Canvas::new(self.horizontal_size, self.vertical_size);
        for y in 0..self.vertical_size - 1 {
            for x in 0..self.horizontal_size - 1 {
                let ray = self.ray_at(x, y);
                let color = world.color_at(&ray);
                canvas.set_pixel(x, y, color);
            }
        }
        canvas
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::{color, float, light, material, matrix, sphere, transform, tuple};
    use crate::color::Color;
    use crate::material::Coloring::SolidColor;
    use crate::object::Object;
    use super::*;

    #[test]
    fn test_pixel_size_for_horizontal_canvas() {
        let view = matrix::IDENTITY;
        let camera = Camera::new(view, 200, 125, PI / 2.);
        assert!(float::is_equal(camera.pixel_size, 0.01));
    }

    #[test]
    fn test_pixel_size_for_vertical_canvas() {
        let view = matrix::IDENTITY;
        let camera = Camera::new(view, 125, 200, PI / 2.);
        assert!(float::is_equal(camera.pixel_size, 0.01));
    }

    #[test]
    fn test_ray_at_center_of_canvas() {
        let view = matrix::IDENTITY;
        let camera = Camera::new(view,201, 101, PI/2.);
        let ray = camera.ray_at(100, 50);
        assert!(ray.origin.is_equal(Tuple::point(0., 0., 0.)));
        assert!(ray.direction.is_equal(Tuple::vector(0., 0., -1.)));
    }

    #[test]
    fn test_ray_at_corner_of_canvas() {
        let view = matrix::IDENTITY;
        let camera = Camera::new(view,201, 101, PI/2.);
        let ray = camera.ray_at(0, 0);
        assert!(ray.origin.is_equal(Tuple::point(0., 0., 0.)));
        assert!(ray.direction.is_equal(Tuple::vector(0.66519, 0.33259, -0.66851)));
    }

    #[test]
    fn test_ray_at_for_transformed_camera() {
        let view = transform::rotation_y(PI/4.).multiply_matrix(transform::translation(0., -2., 5.));
        let camera = Camera::new(view,201, 101, PI/2.);
        let ray = camera.ray_at(100, 50);
        assert!(ray.origin.is_equal(Tuple::point(0., 2., -5.)));
        assert!(ray.direction.is_equal(Tuple::vector(2.0_f64.sqrt()/2.0, 0., -2.0_f64.sqrt()/2.0)));
    }

    pub fn test_world() -> World {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let t1 = matrix::IDENTITY;
        let m1 = material::Material {
            color: SolidColor(color::Color::new(0.8, 1.0, 0.6)),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::DEFAULT_MATERIAL;
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1, s2];
        return World {
            light: light,
            objects: objects,
        };
    }

    #[test]
    fn test_render() {
        let world = test_world();
        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        let view = transform::view(from, to, up);
        let camera = Camera::new(view, 11, 11, PI/2.);
        let canvas = camera.render(world);
        let expected_value = Color::new(0.38066, 0.47583, 0.2855);
        assert_eq!(canvas.get_pixel(5, 5), expected_value);
    }
}