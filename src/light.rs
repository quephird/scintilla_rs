use crate::{color, tuple};

pub struct Light {
    pub intensity: color::Color,
    pub position: tuple::Tuple,
}

impl Light {
    pub fn new(position: tuple::Tuple, intensity: color::Color) -> Light {
        Light {
            intensity: intensity,
            position: position,
        }
    }
}