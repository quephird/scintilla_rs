use crate::color::Color;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::object::Object;
use crate::pattern::Pattern::{CheckerPattern, GradientPattern, RingPattern, StripedPattern};
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone)]
pub enum Pattern {
    StripedPattern(Striped),
    GradientPattern(Gradient),
    RingPattern(Ring),
    CheckerPattern(Checker),
}

impl Pattern {
    pub fn color_at(&self, object: &Object, world_point: Tuple) -> Color {
        let object_point = object.get_inverse_transform().multiply_tuple(world_point);
        let pattern_point = self.get_inverse_transform().multiply_tuple(object_point);
        match self {
            StripedPattern(striped) => striped.color_at(pattern_point),
            GradientPattern(gradient) => gradient.color_at(pattern_point),
            RingPattern(ring) => ring.color_at(pattern_point),
            CheckerPattern(checker) => checker.color_at(pattern_point),
        }
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        match self {
            StripedPattern(striped) => striped.inverse_transform,
            GradientPattern(gradient) => gradient.inverse_transform,
            RingPattern(ring) => ring.inverse_transform,
            CheckerPattern(checker) => checker.inverse_transform,
        }
    }
}

#[derive(Clone)]
pub struct Striped {
    color: Color,
    other_color: Color,
    transform: Matrix4,
    inverse_transform: Matrix4,
}

pub trait PatternMethods {
    fn color_at(&self, point: Tuple) -> Color;
}

impl Striped {
    pub fn new(color: Color, other_color: Color, transform: Matrix4) -> Striped {
        Striped {
            color: color,
            other_color: other_color,
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
        }
    }
}

impl PatternMethods for Striped {
    fn color_at(&self, point: Tuple) -> Color {
        if point[0].floor() % 2. == 0. {
            self.color
        } else {
            self.other_color
        }
    }
}

#[derive(Clone)]
pub struct Gradient {
    color: Color,
    other_color: Color,
    transform: Matrix4,
    inverse_transform: Matrix4,
}

impl Gradient {
    pub fn new(color: Color, other_color: Color, transform: Matrix4) -> Gradient {
        Gradient {
            color: color,
            other_color: other_color,
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
        }
    }
}

impl PatternMethods for Gradient {
    fn color_at(&self, point: Tuple) -> Color {
        let distance = self.other_color.subtract(self.color);
        let fraction = point[0] - point[0].floor();
        return self.color.add(distance.multiply(fraction));
    }
}

#[derive(Clone)]
pub struct Ring {
    color: Color,
    other_color: Color,
    transform: Matrix4,
    inverse_transform: Matrix4,
}

impl Ring {
    pub fn new(color: Color, other_color: Color, transform: Matrix4) -> Ring {
        Ring {
            color: color,
            other_color: other_color,
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
        }
    }
}

impl PatternMethods for Ring {
    fn color_at(&self, point: Tuple) -> Color {
        if (point[0]*point[0] + point[2]*point[2]).sqrt().floor()%2.0 == 0.0 {
            self.color
        } else {
            self.other_color
        }
    }
}

#[derive(Clone)]
pub struct Checker {
    color: Color,
    other_color: Color,
    transform: Matrix4,
    inverse_transform: Matrix4,
}

impl Checker {
    pub fn new(color: Color, other_color: Color, transform: Matrix4) -> Checker {
        Checker {
            color: color,
            other_color: other_color,
            transform: transform,
            inverse_transform: transform.inverse().unwrap(),
        }
    }
}

impl PatternMethods for Checker {
    fn color_at(&self, point: Tuple) -> Color {
        if (point[0].floor() + point[1].floor() + point[2].floor())%2.0 == 0.0 {
            self.color
        } else {
            self.other_color
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{color, matrix, transform};
    use crate::material::Coloring::SurfacePattern;
    use crate::material::Material;
    use crate::sphere::Sphere;
    use crate::object::Object;
    use crate::tuple::TupleMethods;
    use super::*;

    #[test]
    fn test_local_color_at_striped_is_constant_for_y() {
        let pattern = Striped::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 1., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 2., 0.)), color::WHITE);
    }

    #[test]
    fn test_local_color_at_striped_is_constant_for_z() {
        let pattern = Striped::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 1.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 2.)), color::WHITE);
    }

    #[test]
    fn test_local_color_at_striped_alternates_for_x() {
        let pattern = Striped::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.9, 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(1., 0., 0.)), color::BLACK);
        assert_eq!(pattern.color_at(Tuple::point(-0.1, 0., 0.)), color::BLACK);
        assert_eq!(pattern.color_at(Tuple::point(-1., 0., 0.)), color::BLACK);
        assert_eq!(pattern.color_at(Tuple::point(-1.1, 0., 0.)), color::WHITE);
    }

    #[test]
    fn test_world_color_at_with_object_transformation() {
        let pattern = StripedPattern(
            Striped::new(
                color::WHITE,
                color::BLACK,
                matrix::IDENTITY,
            )
        );
        let coloring = SurfacePattern(pattern.clone());
        let material = Material{
            color: coloring,
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let transform = transform::scaling(2., 2., 2.);
        let object = Object::Sphere(
            Sphere::new(transform, material)
        );
        assert_eq!(pattern.color_at(&object, Tuple::point(1.5, 0., 0.)), color::WHITE);
    }

    #[test]
    fn test_world_color_at_with_pattern_transformation() {
        let transform = transform::scaling(2., 2., 2.);
        let pattern = StripedPattern(
            Striped::new(
                color::WHITE,
                color::BLACK,
                transform,
            )
        );
        let coloring = SurfacePattern(pattern.clone());
        let material = Material{
            color: coloring,
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let object = Object::Sphere(
            Sphere::new(matrix::IDENTITY, material)
        );
        assert_eq!(pattern.color_at(&object, Tuple::point(1.5, 0., 0.)), color::WHITE);
    }

    #[test]
    fn test_world_color_at_with_object_and_pattern_transformation() {
        let pattern_transform = transform::translation(0.5, 0.0, 0.0);
        let pattern = StripedPattern(
            Striped::new(
                color::WHITE,
                color::BLACK,
                pattern_transform,
            )
        );
        let coloring = SurfacePattern(pattern.clone());

        let object_transform = transform::scaling(2., 2., 2.);
        let material = Material{
            color: coloring,
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive: 1.0,
        };
        let object = Object::Sphere(
            Sphere::new(object_transform, material)
        );
        assert_eq!(pattern.color_at(&object, Tuple::point(2.5, 0., 0.)), color::WHITE);
    }

    #[test]
    fn test_local_color_at_gradient() {
        let pattern = Gradient::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.25, 0., 0.)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.color_at(Tuple::point(0.5, 0., 0.)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.color_at(Tuple::point(0.75, 0., 0.)), Color::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn test_local_color_at_ring() {
        let pattern = Ring::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(1., 0., 0.)), color::BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 1.)), color::BLACK);
        assert_eq!(pattern.color_at(Tuple::point(0.708, 0., 0.708)), color::BLACK);
    }

    #[test]
    fn test_local_color_at_checker_repeats_for_x() {
        let pattern = Checker::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0.99, 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(1.01, 0., 0.)), color::BLACK);
    }

    #[test]
    fn test_local_color_at_checker_repeats_for_y() {
        let pattern = Checker::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 0.99, 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 1.01, 0.)), color::BLACK);
    }

    #[test]
    fn test_local_color_at_checker_repeats_for_z() {
        let pattern = Checker::new(
            color::WHITE,
            color::BLACK,
            matrix::IDENTITY,
        );
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 0.99)), color::WHITE);
        assert_eq!(pattern.color_at(Tuple::point(0., 0., 1.01)), color::BLACK);
    }
}
