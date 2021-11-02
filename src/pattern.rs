use crate::color::Color;
use crate::matrix::{Matrix4, Matrix4Methods};
use crate::object::Object;
use crate::pattern::Pattern::StripedPattern;
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone)]
pub enum Pattern {
    StripedPattern(Striped),
}

impl Pattern {
    pub fn color_at(&self, object: &Object, world_point: Tuple) -> Color {
        let object_point = object.get_inverse_transform().multiply_tuple(world_point);
        let pattern_point = self.get_inverse_transform().multiply_tuple(object_point);
        match self {
            StripedPattern(striped) => striped.color_at(pattern_point),
        }
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        match self {
            StripedPattern(striped) => striped.inverse_transform,
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

pub trait PatternMethods {
    fn color_at(&self, point: Tuple) -> Color;
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

#[cfg(test)]
mod tests {
    use crate::{color, matrix};
    use crate::tuple::TupleMethods;
    use super::*;

    #[test]
    fn test_color_at_striped_is_constant_for_y() {
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
    fn test_color_at_striped_is_constant_for_z() {
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
    fn test_color_at_striped_alternates_for_x() {
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
}
