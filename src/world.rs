use crate::color::Color;
use crate::intersection::{Computations, Intersection};
use crate::{color, intersection, light};
use crate::light::Light;
use crate::object::Object;
use crate::ray;
use crate::ray::Ray;
use crate::tuple::{Tuple, TupleMethods};

pub fn schlick_reflectance_helper(n1: f64, n2: f64, cosine_of_angle: f64) -> f64 {
    let ratio = (n1 - n2) / (n1 + n2);
    ratio*ratio + (1.0 - ratio*ratio)*(1.0 - cosine_of_angle).powi(5)
}

pub fn schlick_reflectance(computations: Computations) -> f64 {
    // Find the cosine of the angle between the eye and normal vectors
    let cos_theta_1 = computations.eye.dot(computations.normal);
    let n = computations.n1 / computations.n2;
    let sin2_theta_1 = n*n * (1.0 - cos_theta_1*cos_theta_1);
    let cos_theta_2 = (1.0 - sin2_theta_1).sqrt();
    // Total internal reflection can only occur if n1 > n2
    if computations.n1 > computations.n2 && sin2_theta_1 > 1.0 {
        1.0
    } else if computations.n1 > computations.n2 {
        schlick_reflectance_helper(computations.n1, computations.n2, cos_theta_2)
    } else {
        schlick_reflectance_helper(computations.n1, computations.n2, cos_theta_1)
    }
}

pub struct World {
    pub light: light::Light,
    pub objects: Vec<Object>,
}

pub const MAX_RECURSIONS: usize = 5;

impl World {
    pub fn new(light: Light, objects: Vec<Object>) -> World {
        World {
            light: light,
            objects: objects,
        }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Vec<Intersection> {
        let mut all_intersections: Vec<Intersection> = vec![];
        for object in self.objects.iter() {
            let mut intersections = object.intersect(&ray);
            all_intersections.append(&mut intersections)
        }

        all_intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());
        all_intersections
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let light_to_point = self.light.position.subtract(point);
        let distance = light_to_point.magnitude();
        let direction = light_to_point.normalize();
        let ray = Ray::new(point, direction);
        let mut intersections = self.intersect(&ray);
        let hit = intersection::hit(&mut intersections);
        match hit {
            Some(h) => {
                if h.t < distance {
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    pub fn refracted_color(&self, computations: &Computations, remaining_reflections: usize) -> Color {
        if remaining_reflections <= 0 {
            return color::BLACK
        }

        if computations.object.get_material().transparency == 0.0 {
            color::BLACK
        } else {
            // Find the ratio of first index of refraction to the second.
            // (Yup, this is inverted from the definition of Snell's Law.)
            let n_ratio = computations.n1 / computations.n2;
            // cos(theta_i) is the same as the dot product of the two vectors
            let cos_theta_i = computations.eye.dot(computations.normal);
            // Find sin(theta_t)^2 via trigonometric identity
            let sin2_theta_t = n_ratio * n_ratio * (1. - cos_theta_i*cos_theta_i);

            if sin2_theta_t > 1. {
                color::BLACK
            } else {
                // Find cos(theta_t) via trigonometric identity
                let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
                // Compute the direction of the refracted ray
                let direction = computations.normal
                    .multiply(n_ratio * cos_theta_i - cos_theta_t)
                    .subtract(computations.eye.multiply(n_ratio));
                // Create the refracted ray
                let refracted_ray = Ray::new(computations.under_point, direction);
                // Find the color of the refracted ray, making sure to multiply
                // by the transparency value to account for any opacity
                self.color_at(&refracted_ray, remaining_reflections - 1)
                    .multiply(computations.object.get_material().transparency)
            }
        }
    }

    pub fn reflected_color(&self, computations: &Computations, remaining_reflections: usize) -> Color {
        if remaining_reflections <= 0 {
            return color::BLACK
        }

        if computations.object.get_material().reflective == 0.0 {
            color::BLACK
        } else {
            let reflected_ray = Ray::new(computations.over_point, computations.reflected);
            let reflected_color = self.color_at(&reflected_ray, remaining_reflections-1);
            reflected_color.multiply(computations.object.get_material().reflective)
        }
    }

    pub fn shade_hit(&self, computations: Computations, remaining_reflections: usize) -> Color {
        let is_shadowed = self.is_shadowed(computations.over_point);

        let material = computations.object.get_material();
        let surface_color = material.lighting(
            &self.light,
            computations.object,
            computations.point,
            computations.eye,
            computations.normal,
            is_shadowed,
        );
        let reflected_color = self.reflected_color(&computations, remaining_reflections);
        let refracted_color = self.refracted_color(&computations, remaining_reflections);

        if material.reflective > 0. && material.transparency > 0. {
            let reflectance = schlick_reflectance(computations);
            surface_color
                .add(reflected_color.multiply(reflectance))
                .add(refracted_color.multiply(1. - reflectance))
        } else {
            surface_color
                .add(reflected_color)
                .add(refracted_color)
        }
    }

    pub fn color_at(&self, ray: &ray::Ray, remaining_reflections: usize) -> Color {
        let mut intersections = self.intersect(ray);
        // TODO: See if this can be avoided
        let intersections_copy = intersections.clone();
        let hit = intersection::hit(&mut intersections);
        match hit {
            None => color::BLACK,
            Some(intersection) => {
                let computations = intersection.prepare_computations(&ray, intersections_copy);
                self.shade_hit(computations, remaining_reflections)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color, float, matrix, plane};
    use crate::color::Color;
    use crate::intersection::Intersection;
    use crate::light;
    use crate::light::Light;
    use crate::material;
    use crate::material::Coloring::{SolidColor, SurfacePattern};
    use crate::object::Object;
    use crate::pattern::Pattern::TestPattern;
    use crate::pattern::Test;
    use crate::ray::Ray;
    use crate::sphere;
    use crate::transform;
    use crate::tuple;
    use crate::tuple::{Tuple, TupleMethods};
    use crate::world::{MAX_RECURSIONS, schlick_reflectance, World};

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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
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
    fn test_intersect_world() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let intersections = world.intersect(&ray);
        assert_eq!(intersections.len(), 4);
        let ts: Vec<f64> = intersections
            .iter()
            .map(|i| i.t)
            .collect();
        assert_eq!(ts, [4., 4.5, 5.5, 6.]);
    }

    #[test]
    fn test_is_shadowed_point_is_not_collinear_with_light() {
        let world = test_world();
        let point = Tuple::point(0., 10., 0.);
        assert_eq!(world.is_shadowed(point), false);
    }

    #[test]
    fn test_is_shadowed_object_between_light_and_point() {
        let world = test_world();
        let point = Tuple::point(10., -10., 10.);
        assert_eq!(world.is_shadowed(point), true);
    }

    #[test]
    fn test_is_shadowed_light_between_point_and_object() {
        let world = test_world();
        let point = Tuple::point(-20., 20., -20.);
        assert_eq!(world.is_shadowed(point), false);
    }

    #[test]
    fn test_is_shadowed_point_between_light_and_object() {
        let world = test_world();
        let point = Tuple::point(-2., 2., -2.);
        assert_eq!(world.is_shadowed(point), false);
    }

    #[test]
    fn test_shade_hit_outside() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let shape = world.objects.first().unwrap();
        let intersection = Intersection::new(4., shape);
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
        let color = world.shade_hit(computations, MAX_RECURSIONS);
        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shade_hit_inside() {
        let mut world = test_world();
        let light = Light::new(
            Tuple::point(0., 0.25, 0.),
            Color::new(1., 1., 1.),
        );
        world.light = light;
        let ray = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.)
        );
        let shape = world.objects.iter().nth(1).unwrap();
        let intersection = Intersection::new(0.5, shape);
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
        let color = world.shade_hit(computations, MAX_RECURSIONS);
        assert_eq!(color, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_shade_hit_reflective_material() {
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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };

        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 1.0,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let t3 = transform::translation(0., -1., 0.);
        let m3 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.5,
            transparency: 0.0,
            refractive: 1.0,
        };
        let plane = Object::Plane(
            plane::Plane::new(t3, m3)
        );

        let objects = vec![s1.clone(), s2.clone(), plane.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.)
        );
        let intersection = Intersection::new(2.0_f64.sqrt(), &plane);
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
        let color = world.shade_hit(computations, MAX_RECURSIONS);
        assert_eq!(color, Color::new(0.87676, 0.92434, 0.82917));
    }

    #[test]
    fn test_color_at_ray_misses() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 1., 0.)
        );
        let color = world.color_at(&ray, MAX_RECURSIONS);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn test_color_at_ray_hits() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let color = world.color_at(&ray, MAX_RECURSIONS);
        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_color_at_ray_inside_outer_sphere_and_outside_inner_sphere() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let t1 = matrix::IDENTITY;
        let m1 = material::Material {
            color: SolidColor(color::Color::new(0.8, 1.0, 0.6)),
            ambient: 1.,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let mut m2 = material::DEFAULT_MATERIAL;
        m2.ambient = 1.;
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1, s2];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., 0.75),
            Tuple::vector(0., 0., -1.)
        );
        let color = world.color_at(&ray, MAX_RECURSIONS);
        assert_eq!(color, color::WHITE);
    }

    #[test]
    fn test_prepare_computations_nonrelective_material() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let t1 = matrix::IDENTITY;
        let m1 = material::Material {
            color: SolidColor(color::Color::new(0.8, 1.0, 0.6)),
            ambient: 0.0,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };

        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1.clone(), s2.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 0., 1.)
        );
        let intersection = Intersection::new(1., &s2);
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
        let reflected_color = world.reflected_color(&computations, MAX_RECURSIONS);
        assert_eq!(reflected_color, color::BLACK);
    }

    #[test]
    fn test_prepare_computations_reflective_material() {
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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };

        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 1.0,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let t3 = transform::translation(0., -1., 0.);
        let m3 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.0,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            reflective: 0.5,
            transparency: 0.0,
            refractive: 1.0,
        };
        let plane = Object::Plane(
            plane::Plane::new(t3, m3)
        );

        let objects = vec![s1.clone(), s2.clone(), plane.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.)
        );
        let intersection = Intersection::new(2.0_f64.sqrt(), &plane);
        let computations = intersection.prepare_computations(
            &ray, vec![intersection.clone()]
        );
        let reflected_color = world.reflected_color(&computations, MAX_RECURSIONS);
        assert_eq!(reflected_color, Color::new(0.19033, 0.23792, 0.14275));
    }

    #[test]
    fn test_color_at_terminates_safely() {
        let light = light::Light::new(
            tuple::Tuple::point(0., 0., 0.),
            color::Color::new(1., 1., 1.)
        );
        let t1 = transform::translation(0., -1., 0.);
        let m1 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            reflective: 1.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let lower_plane = Object::Plane(
            plane::Plane::new(t1, m1)
        );

        let t2 = transform::translation(0., 1., 0.);
        let m2 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0,
            reflective: 1.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let upper_plane = Object::Plane(
            plane::Plane::new(t2, m2)
        );

        let objects = vec![lower_plane, upper_plane];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.)
        );
        // There is nothing to assert here; just that the call to color_at terminates.
        let _color = world.color_at(&ray, MAX_RECURSIONS);
    }

    #[test]
    fn test_refracted_color_opaque_surface() {
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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::DEFAULT_MATERIAL;
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1.clone(), s2.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let intersections = world.intersect(&ray);
        let i1 = intersections.iter().nth(0).unwrap();
        let computations = i1.prepare_computations(&ray, intersections.clone());
        let color = world.refracted_color(&computations, MAX_RECURSIONS);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn test_refracted_color_at_maximum_recursive_depth() {
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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 1.0,
            refractive: 1.5,
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::DEFAULT_MATERIAL;
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1.clone(), s2.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let intersections = world.intersect(&ray);
        let i1 = intersections.iter().nth(0).unwrap();
        let computations = i1.prepare_computations(&ray, intersections.clone());
        let color = world.refracted_color(&computations, 0);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn test_refracted_color_total_internal_reflection() {
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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 1.0,
            refractive: 1.5,
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::DEFAULT_MATERIAL;
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1.clone(), s2.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., 2.0_f64.sqrt() / 2.),
            Tuple::vector(0., 1., 0.)
        );
        let intersections = world.intersect(&ray);
        // NOTE: this time you're inside the sphere, so you need
        // to look at the second intersection not the first one.
        let i2 = intersections.iter().nth(1).unwrap();
        let computations = i2.prepare_computations(&ray, intersections.clone());
        let color = world.refracted_color(&computations, MAX_RECURSIONS);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn test_refracted_color_for_refracted_ray() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let t1 = matrix::IDENTITY;
        let m1 = material::Material {
            color: SurfacePattern(TestPattern(Test::new(matrix::IDENTITY))),
            ambient: 1.0,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 1.0,
            refractive: 1.5,
        };
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![s1.clone(), s2.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., 0.1),
            Tuple::vector(0., 1., 0.)
        );
        let intersections = world.intersect(&ray);
        let i3 = intersections.iter().nth(2).unwrap();
        let computations = i3.prepare_computations(&ray, intersections.clone());
        let color = world.refracted_color(&computations, MAX_RECURSIONS);
        assert_eq!(color, Color::new(0., 0.99888, 0.04722));
    }

    #[test]
    fn test_shade_hit_for_transparent_material() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let t1 = transform::translation(0., -1., 0.);
        let m1 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.5,
            refractive: 1.5,
        };
        let floor = Object::Plane(
            plane::Plane::new(t1, m1)
        );

        let t2 = transform::translation(0.0, -3.5, -0.5);
        let m2 = material::Material {
            color: SolidColor(Color::new(1., 0., 0.)),
            ambient: 0.5,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let ball = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let objects = vec![floor.clone(), ball.clone()];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt()/2., 2.0_f64.sqrt()/2.)
        );
        let intersections = world.intersect(&ray);
        let i0 = intersections.iter().nth(0).unwrap();
        let computations = i0.prepare_computations(&ray, intersections.clone());
        let color = world.shade_hit(computations, MAX_RECURSIONS);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn test_schlick_reflectance_total_internal_reflection() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let glass = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 1.0,
            refractive: 1.5,
        };
        let glassy_sphere = Object::Sphere(
            sphere::Sphere::new(
                matrix::IDENTITY,
                glass
            )
        );
        let world = World {
            light: light,
            objects: vec![glassy_sphere],
        };

        let ray = Ray::new(
            Tuple::point(0., 0., 2.0_f64.sqrt()/2.),
            Tuple::vector(0., 1., 0.)
        );
        let intersections = world.intersect(&ray);
        let i1 = intersections.iter().nth(1).unwrap();
        let computations = i1.prepare_computations(&ray, intersections.clone());
        let reflectance = schlick_reflectance(computations);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn test_schlick_reflectance_perpendicular_ray() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let glass = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 1.0,
            refractive: 1.5,
        };
        let glassy_sphere = Object::Sphere(
            sphere::Sphere::new(
                matrix::IDENTITY,
                glass
            )
        );
        let world = World {
            light: light,
            objects: vec![glassy_sphere],
        };

        let ray = Ray::new(
            Tuple::point(0., 0., 0.),
            Tuple::vector(0., 1., 0.)
        );
        let intersections = world.intersect(&ray);
        let i1 = intersections.iter().nth(1).unwrap();
        let computations = i1.prepare_computations(&ray, intersections.clone());
        let reflectance = schlick_reflectance(computations);
        assert!(float::is_equal(reflectance, 0.04));
    }

    #[test]
    fn test_schlick_reflectance_small_angle() {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let glass = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 1.0,
            refractive: 1.5,
        };
        let glassy_sphere = Object::Sphere(
            sphere::Sphere::new(
                matrix::IDENTITY,
                glass
            )
        );
        let world = World {
            light: light,
            objects: vec![glassy_sphere],
        };

        let ray = Ray::new(
            Tuple::point(0., 0.99, -2.),
            Tuple::vector(0., 0., 1.)
        );
        let intersections = world.intersect(&ray);
        let i0 = intersections.iter().nth(0).unwrap();
        let computations = i0.prepare_computations(&ray, intersections.clone());
        let reflectance = schlick_reflectance(computations);
        assert!(float::is_equal(reflectance, 0.48881));
    }

    #[test]
    fn test_shade_hit_with_schlick_reflectance() {
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
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let s1 = Object::Sphere(
            sphere::Sphere::new(t1, m1)
        );

        let t2 = transform::scaling(0.5, 0.5, 0.5);
        let m2 = material::DEFAULT_MATERIAL;
        let s2 = Object::Sphere(
            sphere::Sphere::new(t2, m2)
        );

        let t3 = transform::translation(0., -1., 0.);
        let m3 = material::Material {
            color: SolidColor(color::WHITE),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.5,
            transparency: 0.5,
            refractive: 1.5,
        };
        let floor = Object::Plane(
            plane::Plane::new(t3, m3)
        );

        let t4 = transform::translation(0.0, -3.5, -0.5);
        let m4 = material::Material {
            color: SolidColor(Color::new(1., 0., 0.)),
            ambient: 0.5,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 0.0,
        };
        let ball = Object::Sphere(
            sphere::Sphere::new(t4, m4)
        );

        let objects = vec![s1, s2, ball, floor];
        let world = World {
            light: light,
            objects: objects,
        };

        let ray = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., -2.0_f64.sqrt()/2., 2.0_f64.sqrt()/2.)
        );
        let intersections = world.intersect(&ray);
        let i0 = intersections.iter().nth(0).unwrap();
        let computations = i0.prepare_computations(&ray, intersections.clone());
        let color = world.shade_hit(computations, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
