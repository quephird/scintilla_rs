use crate::{color, light, tuple};

#[derive(Clone)]
pub struct Material {
    pub color: color::Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

pub const DEFAULT_MATERIAL:Material = Material {
    color: [1., 1., 1.],
    ambient: 0.1,
    diffuse: 0.9,
    specular: 0.9,
    shininess: 200.0,
};

impl Material {
    pub fn new() -> Material {
        DEFAULT_MATERIAL
    }

    pub fn lighting(&self,
                    light: &light::Light,
                    point: tuple::Tuple,
                    eye: tuple::Tuple,
                    normal: tuple::Tuple) -> color::Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = color::hadamard(self.color, light.intensity);

        // Find the direction to the light source
        let light_vector = tuple::normalize(tuple::subtract(light.position, point));

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = tuple::dot(light_vector, normal);

        let ambient = color::multiply(effective_color, self.ambient);
        let diffuse: color::Color;
        let specular: color::Color;

        if light_dot_normal < 0. {
            diffuse = color::BLACK;
            specular = color::BLACK;
        } else {
            // Compute the diffuse contribution
            diffuse = color::multiply(effective_color, self.diffuse * light_dot_normal);
            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflected = tuple::reflect(tuple::negate(light_vector), normal);
            let reflected_dot_eye = tuple::dot(reflected, eye);

            if reflected_dot_eye <= 0. {
                specular = color::BLACK;
            } else {
                // Compute the specular contribution
                let factor = reflected_dot_eye.powf(self.shininess);
                specular = color::multiply(light.intensity,self.specular * factor);
            }
        }

        // Add the three contributions together to get the final shading
        color::add(color::add(ambient, diffuse), specular)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lighting_eye_between_light_and_surface() {
        let material = Material::new();
        let position = tuple::point(0., 0., 0.);
        let eye = tuple::vector(0., 0., -1.);
        let normal = tuple::vector(0., 0., -1.);
        let light = light::Light::new(tuple::point(0., 0., -10.), [1., 1., 1.]);
        let color = material.lighting(&light, position, eye, normal);
        assert!(color::is_equal(color, [1.9, 1.9, 1.9]));
    }

    #[test]
    fn test_lighting_eye_offset_45_degrees() {
        let material = Material::new();
        let position = tuple::point(0., 0., 0.);
        let eye = tuple::vector(0., 2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normal = tuple::vector(0., 0., -1.);
        let light = light::Light::new(tuple::point(0., 0., -10.), [1., 1., 1.]);
        let color = material.lighting(&light, position, eye, normal);
        assert!(color::is_equal(color, [1.0, 1.0, 1.0]));
    }

    #[test]
    fn test_lighting_light_offset_45_degrees() {
        let material = Material::new();
        let position = tuple::point(0., 0., 0.);
        let eye = tuple::vector(0., 0., -1.);
        let normal = tuple::vector(0., 0., -1.);
        let light = light::Light::new(tuple::point(0., 10., -10.), [1., 1., 1.]);
        let color = material.lighting(&light, position, eye, normal);
        assert!(color::is_equal(color, [0.7364, 0.7364, 0.7364]));
    }

    #[test]
    fn test_lighting_eye_in_reflection_path() {
        let material = Material::new();
        let position = tuple::point(0., 0., 0.);
        let eye = tuple::vector(0., -2.0_f64.sqrt() / 2., -2.0_f64.sqrt() / 2.);
        let normal = tuple::vector(0., 0., -1.);
        let light = light::Light::new(tuple::point(0., 10., -10.), [1., 1., 1.]);
        let color = material.lighting(&light, position, eye, normal);
        assert!(color::is_equal(color, [1.6364, 1.6364, 1.6364]));
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let material = Material::new();
        let position = tuple::point(0., 0., 0.);
        let eye = tuple::vector(0., 0., -1.);
        let normal = tuple::vector(0., 0., -1.);
        let light = light::Light::new(tuple::point(0., 0., 10.), [1., 1., 1.]);
        let color = material.lighting(&light, position, eye, normal);
        assert!(color::is_equal(color, [0.1, 0.1, 0.1]));
    }
}
