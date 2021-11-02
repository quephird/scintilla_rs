use crate::color::Color;
use crate::matrix::Matrix4;
use crate::pattern::Pattern::StripedPattern;
use crate::tuple::Tuple;

#[derive(Clone)]
pub enum Pattern {
    StripedPattern(Striped),
}

impl Pattern {
    pub fn color_at(&self, world_point: Tuple) -> Color {
        match self {
            StripedPattern(striped) => striped.color_at(world_point),
        }
    }
}

#[derive(Clone)]
pub struct Striped {
    color: Color,
    other_color: Color,
    transform: Matrix4,
}

impl Striped {
    pub fn new(color: Color, other_color: Color, transform: Matrix4) -> Striped {
        Striped {
            color: color,
            other_color: other_color,
            transform: transform,
        }
    }
}

pub trait PatternMethods {
    fn color_at(&self, world_point: Tuple) -> Color;
}

impl PatternMethods for Striped {
    fn color_at(&self, world_point: Tuple) -> Color {
        if world_point[0].floor() % 2. == 0. {
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
