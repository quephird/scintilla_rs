use crate::float;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub const BLACK: Color = Color{r: 0.0, g: 0.0, b: 0.0};
pub const WHITE: Color = Color{r: 1.0, g: 1.0, b: 1.0};

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color{r: r, g: g, b: b}
    }

    pub fn add(&self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }

    pub fn subtract(&self, other: Color) -> Color {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }

    pub fn multiply(&self, s: f64) -> Color {
        Color::new(self.r * s, self.g * s, self.b * s)
    }

    pub fn hadamard(&self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

    pub fn is_equal(&self, other: Color) -> bool {
        float::is_equal(self.r, other.r) &&
            float::is_equal(self.g, other.g) &&
            float::is_equal(self.b, other.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(c1.add(c2).is_equal(Color::new(1.6, 0.7, 1.)));
    }

    #[test]
    fn test_subtract() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(c1.subtract(c2).is_equal(Color::new(0.2, 0.5, 0.5)));
    }

    #[test]
    fn test_multiply() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert!(c.multiply(2.).is_equal(Color::new(0.4, 0.6, 0.8)));
    }

    #[test]
    fn test_hadamard() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);
        assert!(c1.hadamard(c2).is_equal(Color::new(0.9, 0.2, 0.04)));
    }
}
