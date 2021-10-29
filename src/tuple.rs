use crate::float;

pub type Tuple = [f64; 4];

pub trait TupleMethods {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple;
    fn point(x: f64, y: f64, z: f64) -> Tuple;
    fn vector(x: f64, y: f64, z: f64) -> Tuple;
    fn is_equal(&self, other: Tuple) -> bool;
    fn add(&self, other: Tuple) -> Tuple;
    fn subtract(&self, other: Tuple) -> Tuple;
    fn multiply(&self, s: f64) -> Tuple;
    fn negate(&self) -> Tuple;
    fn divide(&self, s: f64) -> Tuple;
    fn magnitude(&self) -> f64;
    fn dot(&self, other: Tuple) -> f64;
    fn normalize(&self) -> Tuple;
    fn cross(&self, other: Tuple) -> Tuple;
    fn reflect(&self, normal: Tuple) -> Tuple;
}

impl TupleMethods for Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        [x, y, z, w]
    }

    fn point(x: f64, y: f64, z: f64) -> Tuple {
        [x, y, z, 1.]
    }

    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        [x, y, z, 0.]
    }

    fn is_equal(&self, other: Tuple) -> bool {
        float::is_equal(self[0], other[0]) &&
            float::is_equal(self[1], other[1]) &&
            float::is_equal(self[2], other[2]) &&
            float::is_equal(self[3], other[3])
    }

    fn add(&self, other: Tuple) -> Tuple {
        [
            self[0]+other[0],
            self[1]+other[1],
            self[2]+other[2],
            self[3]+other[3]
        ]
    }

    fn subtract(&self, other: Tuple) -> Tuple {
        [
            self[0]-other[0],
            self[1]-other[1],
            self[2]-other[2],
            self[3]-other[3]
        ]
    }

    fn multiply(&self, s: f64) -> Tuple {
        [s*self[0], s*self[1], s*self[2], s*self[3]]
    }

    fn negate(&self) -> Tuple {
        [-self[0], -self[1], -self[2], -self[3]]
    }

    fn divide(&self, s: f64) -> Tuple {
        [self[0]/s, self[1]/s, self[2]/s, self[3]/s]
    }

    fn magnitude(&self) -> f64 {
        (self[0]*self[0] + self[1]*self[1] + self[2]*self[2]).sqrt()
    }

    fn dot(&self, other: Tuple) -> f64 {
        self[0]*other[0] + self[1]*other[1] + self[2]*other[2] + self[3]*other[3]
    }

    fn normalize(&self) -> Tuple {
        self.divide(self.magnitude())
    }

    fn cross(&self, other: Tuple) -> Tuple {
        [
            self[1]*other[2] - self[2]*other[1],
            self[2]*other[0] - self[0]*other[2],
            self[0]*other[1] - self[1]*other[0],
            0.
        ]
    }

    fn reflect(&self, normal: Tuple) -> Tuple {
        self.subtract(normal.multiply(2. * self.dot(normal)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equal() {
        assert!([1., 2., 3., 0.].is_equal([1., 2., 3., 0.]));
        assert!([1., 2., 3., 0.].is_equal([1.000005, 1.999995, 2.999995, 0.]));

        assert!([1., 2., 3., 0.].is_equal([1., 2., 3., 0.]));
        assert!([1., 2., 3., 0.].is_equal([1.000005, 1.999995, 2.999995, 0.]));
    }

    #[test]
    fn test_add() {
        let t1: Tuple = [3., -2., 5., 1.];
        let t2: Tuple = [-2., 3., 1., 0.];
        assert_eq!(t1.add(t2).is_equal([1., 1., 6., 1.]), true);
    }

    #[test]
    fn test_subtract() {
        let t1: Tuple = [3., 2., 1., 1.];
        let t2: Tuple = [5., 6., 7., 1.];
        assert_eq!(t1.subtract(t2).is_equal([-2., -4., -6., 0.]), true);

        let t3: Tuple = [3., 2., 1., 1.];
        let t4: Tuple = [5., 6., 7., 0.];
        assert_eq!(t3.subtract(t4).is_equal([-2., -4., -6., 1.]), true);

        let t3: Tuple = [3., 2., 1., 0.];
        let t4: Tuple = [5., 6., 7., 0.];
        assert_eq!(t3.subtract(t4).is_equal([-2., -4., -6., 0.]), true);
    }

    #[test]
    fn test_negate() {
        assert_eq!([1., -2., 3., -4.].negate().is_equal([-1., 2., -3., 4.]), true);
    }

    #[test]
    fn test_multiply() {
        assert!([1., -2., 3., -4.].multiply(3.5).is_equal([3.5, -7., 10.5, -14.]));
        assert!([1., -2., 3., -4.].multiply(0.5).is_equal([0.5, -1., 1.5, -2.]));
    }

    #[test]
    fn test_divide() {
        assert!([1., -2., 3., -4.].divide(2.).is_equal([0.5, -1., 1.5, -2.]));
    }

    #[test]
    fn test_magnitude() {
        assert!(float::is_equal([1., 0., 0., 0.].magnitude(), 1.0));
        assert!(float::is_equal([0., 1., 0., 0.].magnitude(), 1.0));
        assert!(float::is_equal([0., 0., 1., 0.].magnitude(), 1.0));

        assert!(float::is_equal([1., 2., 3., 0.].magnitude(), 14.0_f64.sqrt()));

        assert!(float::is_equal([-1., -2., -3., 0.].magnitude(), 14.0_f64.sqrt()));
    }

    #[test]
    fn test_normalize() {
        assert!([4., 0., 0., 0.].normalize().is_equal([1., 0., 0., 0.]));

        assert!([1., 2., 3., 0.].normalize().is_equal([0.26726, 0.53452, 0.80178, 0.]));

        let n = [1., 2., 3., 0.].normalize();
        assert!(float::is_equal(n.magnitude(), 1.));
    }

    #[test]
    fn test_dot() {
        let t1: Tuple = [1., 2., 3., 0.];
        let t2: Tuple = [2., 3., 4., 0.];
        assert_eq!(float::is_equal(t1.dot(t2), 20.), true);
    }

    #[test]
    fn test_cross() {
        let t1: Tuple = [1., 2., 3., 0.];
        let t2: Tuple = [2., 3., 4., 0.];
        assert!(t1.cross(t2).is_equal([-1., 2., -1., 0.]));

        assert!(t2.cross(t1).is_equal([1., -2., 1., 0.]));
    }

    #[test]
    fn test_reflect_45_degrees() {
        let incident = Tuple::vector(1., -1., 0.);
        let normal = Tuple::vector(0., 1., 0.);
        let reflected = incident.reflect(normal);
        assert!(reflected.is_equal(Tuple::vector(1., 1., 0.)));
    }

    #[test]
    fn test_reflect_slanted_surface() {
        let incident = Tuple::vector(0., -1., 0.);
        let normal = Tuple::vector(2_f64.sqrt()/2., 2_f64.sqrt()/2., 0.);
        let reflected = incident.reflect(normal);
        assert!(reflected.is_equal(Tuple::vector(1., 0., 0.)));
    }
}
