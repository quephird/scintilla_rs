use crate::{color, light, material, pattern, tuple};
use crate::color::Color;
use crate::material::Coloring::{SolidColor, SurfacePattern};
use crate::object::Object;
use crate::pattern::Pattern;
use crate::pattern::PatternMethods;
use crate::shape::Shape;
use crate::tuple::TupleMethods;

#[derive(Clone)]
pub enum Coloring {
    SolidColor(Color),
    SurfacePattern(Pattern),
}

#[derive(Clone)]
pub struct Material {
    pub color: Coloring,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive: f64,
}

pub const DEFAULT_MATERIAL:Material = Material {
    color: SolidColor(color::WHITE),
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
    reflective: 0.0,
    transparency: 0.0,
    refractive: 1.0,
};

impl Material {
    pub fn new() -> Material {
        DEFAULT_MATERIAL
    }

    pub fn lighting(&self,
                    light: &light::Light,
                    object: &Object,
                    point: tuple::Tuple,
                    eye: tuple::Tuple,
                    normal: tuple::Tuple,
                    is_shadowed: bool) -> color::Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = match &self.color {
            SolidColor(color) => *color,
            SurfacePattern(pattern) => pattern.color_at(object, point),
        }.hadamard(light.intensity);
        let ambient = effective_color.multiply(self.ambient);

        if is_shadowed == true {
            ambient
        } else {
            // Find the direction to the light source
            let light_vector = light.position.subtract(point).normalize();

            // light_dot_normal represents the cosine of the angle between the
            // light vector and the normal vector. A negative number means the
            // light is on the other side of the surface.
            let light_dot_normal = light_vector.dot(normal);

            let diffuse: color::Color;
            let specular: color::Color;

            if light_dot_normal < 0. {
                diffuse = color::BLACK;
                specular = color::BLACK;
            } else {
                // Compute the diffuse contribution
                diffuse = effective_color.multiply(self.diffuse * light_dot_normal);
                // reflect_dot_eye represents the cosine of the angle between the
                // reflection vector and the eye vector. A negative number means the
                // light reflects away from the eye.
                let reflected = light_vector.negate().reflect(normal);
                let reflected_dot_eye = reflected.dot(eye);

                if reflected_dot_eye <= 0. {
                    specular = color::BLACK;
                } else {
                    // Compute the specular contribution
                    let factor = reflected_dot_eye.powf(self.shininess);
                    specular = light.intensity.multiply(self.specular * factor);
                }
            }

            // Add the three contributions together to get the final shading
            ambient.add(diffuse).add(specular)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::light::Light;
    use crate::matrix;
    use crate::pattern::Pattern::StripedPattern;
    use crate::pattern::Striped;
    use crate::sphere::Sphere;
    use crate::tuple::Tuple;
    use super::*;

    #[test]
    fn test_lighting_eye_between_light_and_surface() {
        let material = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vector(0., 0., -1.);
        let normal = Tuple::vector(0., 0., -1.);
        let light = light::Light::new(Tuple::point(0., 0., -10.), color::WHITE);
        let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material::DEFAULT_MATERIAL,
            )
        );
        let color = material.lighting(&light, &sphere, position, eye, normal, false);
        assert_eq!(color, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_eye_offset_45_degrees() {
        let material = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vector(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normal = Tuple::vector(0., 0., -1.);
        let light = light::Light::new(Tuple::point(0., 0., -10.), color::WHITE);
        let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material::DEFAULT_MATERIAL,
            )
        );
        let color = material.lighting(&light, &sphere, position, eye, normal, false);
        assert_eq!(color, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_light_offset_45_degrees() {
        let material = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vector(0., 0., -1.);
        let normal = Tuple::vector(0., 0., -1.);
        let light = light::Light::new(Tuple::point(0., 10., -10.), color::WHITE);
        let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material::DEFAULT_MATERIAL,
            )
        );
        let color = material.lighting(&light, &sphere,position, eye, normal, false);
        assert_eq!(color, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_eye_in_reflection_path() {
        let material = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vector(0., -2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normal = Tuple::vector(0., 0., -1.);
        let light = light::Light::new(Tuple::point(0., 10., -10.), color::WHITE);
        let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material::DEFAULT_MATERIAL,
            )
        );
        let color = material.lighting(&light, &sphere, position, eye, normal, false);
        assert_eq!(color, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let material = Material::new();
        let position = Tuple::point(0., 0., 0.);
        let eye = Tuple::vector(0., 0., -1.);
        let normal = Tuple::vector(0., 0., -1.);
        let light = light::Light::new(Tuple::point(0., 0., 10.), color::WHITE);
        let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material::DEFAULT_MATERIAL,
            )
        );
        let color = material.lighting(&light, &sphere, position, eye, normal, false);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_pattern() {
        let pattern = Striped::new(
                color::WHITE,
                color::BLACK,
                matrix::IDENTITY,
        );
        let material = Material {
            color: Coloring::SurfacePattern(StripedPattern(pattern)),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let sphere = Object::Sphere(
            Sphere::new(
                matrix::IDENTITY,
                material::DEFAULT_MATERIAL,
            )
        );
        let eye = Tuple::vector(0., 0., -1.);
        let normal = Tuple::vector(0., 0., -1.);
        let light = Light::new(
            Tuple::point(0., 0., -10.),
            Color::new(1., 1., 1.)
        );
        let p1 = Tuple::point(0.9, 0., 0.);
        let c1 = material.lighting(&light, &sphere, p1, eye, normal, false);
        assert_eq!(c1, color::WHITE);

        let p2 = Tuple::point(1.1, 0., 0.);
        let c2 = material.lighting(&light, &sphere, p2, eye, normal, false);
        assert_eq!(c2, color::BLACK);
    }
}
