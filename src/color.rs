use crate::float;

pub type Color = [f64; 3];

pub const BLACK: Color = [0.0, 0.0, 0.0];

pub fn add(c1: Color, c2: Color) -> Color {
    [c1[0]+c2[0], c1[1]+c2[1], c1[2]+c2[2]]
}

pub fn subtract(c1: Color, c2: Color) -> Color {
    [c1[0]-c2[0], c1[1]-c2[1], c1[2]-c2[2]]
}

pub fn multiply(c: Color, s: f64) -> Color {
    [s*c[0], s*c[1], s*c[2]]
}

pub fn hadamard(c1: Color, c2: Color) -> Color {
    [c1[0]*c2[0], c1[1]*c2[1], c1[2]*c2[2]]
}

pub fn is_equal(c1: Color, c2: Color) -> bool {
    float::is_equal(c1[0], c2[0]) &&
        float::is_equal(c1[1], c2[1]) &&
        float::is_equal(c1[2], c2[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = [0.9, 0.6, 0.75];
        let c2 = [0.7, 0.1, 0.25];
        assert_eq!(is_equal(add(c1, c2), [1.6, 0.7, 1.]), true);
    }

    #[test]
    fn test_subtract() {
        let c1 = [0.9, 0.6, 0.75];
        let c2 = [0.7, 0.1, 0.25];
        assert_eq!(is_equal(subtract(c1, c2), [0.2, 0.5, 0.5]), true);
    }

    #[test]
    fn test_multiply() {
        let c1 = [0.2, 0.3, 0.4];
        assert_eq!(is_equal(multiply(c1, 2.), [0.4, 0.6, 0.8]), true);
    }

    #[test]
    fn test_hadamard() {
        let c1 = [1., 0.2, 0.4];
        let c2 = [0.9, 1., 0.1];
        assert_eq!(is_equal(hadamard(c1, c2), [0.9, 0.2, 0.04]), true);
    }
}
