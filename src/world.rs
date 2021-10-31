use crate::color::Color;
use crate::intersection::{Computations, Intersection};
use crate::{color, intersection, light};
use crate::object::Object;
use crate::ray;

pub struct World {
    pub light: light::Light,
    pub objects: Vec<Object>,
}

impl World {
    pub fn intersect(&self, ray: &ray::Ray) -> Vec<Intersection> {
        let mut all_intersections: Vec<Intersection> = vec![];
        for object in self.objects.iter() {
            let mut intersections = object.intersect(&ray);
            all_intersections.append(&mut intersections)
        }

        all_intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());
        all_intersections
    }

    pub fn shade_hit(&self, computations: Computations) -> Color {
        let material = computations.object.get_material();
        material.lighting(
            &self.light,
            computations.point,
            computations.eye,
            computations.normal
        )
    }

    pub fn color_at(&self, ray: &ray::Ray) -> Color {
        let mut intersections = self.intersect(ray);
        let hit = intersection::hit(&mut intersections);
        match hit {
            None => color::BLACK,
            Some(intersection) => {
                let computations = intersection.prepare_computations(&ray);
                self.shade_hit(computations)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color, matrix};
    use crate::color::Color;
    use crate::intersection::Intersection;
    use crate::light;
    use crate::light::Light;
    use crate::material;
    use crate::object::Object;
    use crate::ray::Ray;
    use crate::sphere;
    use crate::transform;
    use crate::tuple;
    use crate::tuple::{Tuple, TupleMethods};
    use crate::world::World;

    pub fn test_world() -> World {
        let light = light::Light::new(
            tuple::Tuple::point(-10., 10., -10.),
            color::Color::new(1., 1., 1.)
        );

        let t1 = matrix::IDENTITY;
        let m1 = material::Material {
            color: color::Color::new(0.8, 1.0, 0.6),
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
    fn test_shade_hit_outside() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let shape = world.objects.first().unwrap();
        let intersection = Intersection::new(4., shape);
        let computations = intersection.prepare_computations(&ray);
        let color = world.shade_hit(computations);
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
        let computations = intersection.prepare_computations(&ray);
        let color = world.shade_hit(computations);
        assert_eq!(color, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_color_at_ray_misses() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 1., 0.)
        );
        let color = world.color_at(&ray);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn test_color_at_ray_hits() {
        let world = test_world();
        let ray = Ray::new(
            Tuple::point(0., 0., -5.),
            Tuple::vector(0., 0., 1.)
        );
        let color = world.color_at(&ray);
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
            color: color::Color::new(0.8, 1.0, 0.6),
            ambient: 1.,
            diffuse: 0.7,
            specular: 0.2,
            shininess: 200.0
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
        let color = world.color_at(&ray);
        assert_eq!(color, color::WHITE);
    }
}
