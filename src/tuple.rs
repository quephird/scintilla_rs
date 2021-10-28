use crate::float;

pub type Tuple = [f64; 4];

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    [x, y, z, 1.]
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    [x, y, z, 0.]
}

pub fn is_point(t: Tuple) -> bool {
    float::is_equal(t[3], 1.0)
}

pub fn is_equal(t1: Tuple, t2: Tuple) -> bool {
    float::is_equal(t1[0], t2[0]) &&
        float::is_equal(t1[1], t2[1]) &&
        float::is_equal(t1[2], t2[2]) &&
        float::is_equal(t1[3], t2[3])
}

pub fn add(t1: Tuple, t2: Tuple) -> Tuple {
    [t1[0]+t2[0], t1[1]+t2[1], t1[2]+t2[2], t1[3]+t2[3]]
}

pub fn subtract(t1: Tuple, t2: Tuple) -> Tuple {
    [t1[0]-t2[0], t1[1]-t2[1], t1[2]-t2[2], t1[3]-t2[3]]
}

pub fn negate(t1: Tuple) -> Tuple {
    [-t1[0], -t1[1], -t1[2], -t1[3]]
}

pub fn multiply(t: Tuple, s: f64) -> Tuple {
    [s*t[0], s*t[1], s*t[2], s*t[3]]
}

pub fn divide(t: Tuple, s: f64) -> Tuple {
    [t[0]/s, t[1]/s, t[2]/s, t[3]/s]
}

pub fn magnitude(t: Tuple) -> f64 {
    (t[0]*t[0] + t[1]*t[1] + t[2]*t[2]).sqrt()
}

pub fn normalize(t: Tuple) -> Tuple {
    divide(t, magnitude(t))
}

pub fn dot(t1: Tuple, t2: Tuple) -> f64 {
    t1[0]*t2[0] + t1[1]*t2[1] + t1[2]*t2[2] + t1[3]*t2[3]
}

pub fn cross(t1: Tuple, t2: Tuple) -> Tuple {
    [
        t1[1]*t2[2] - t1[2]*t2[1],
        t1[2]*t2[0] - t1[0]*t2[2],
        t1[0]*t2[1] - t1[1]*t2[0],
        0.
    ]
}

pub fn reflect(incident: Tuple, normal: Tuple) -> Tuple {
    subtract(incident, multiply(normal, 2. * dot(incident, normal)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_point() {
        assert!(is_point([1., 2., 3., 1.]));

        assert!(!is_point([1., 2., 3., 0.]));
    }

    #[test]
    fn test_is_equal() {
        assert!(is_equal([1., 2., 3., 0.], [1., 2., 3., 0.]));
        assert!(is_equal([1., 2., 3., 0.], [1.000005, 1.999995, 2.999995, 0.]));

        assert!(is_equal([1., 2., 3., 0.], [1., 2., 3., 0.]));
        assert!(is_equal([1., 2., 3., 0.], [1.000005, 1.999995, 2.999995, 0.]));
    }

    #[test]
    fn test_add() {
        let t1: Tuple = [3., -2., 5., 1.];
        let t2: Tuple = [-2., 3., 1., 0.];
        assert_eq!(is_equal(add(t1, t2), [1., 1., 6., 1.]), true);
    }

    #[test]
    fn test_subtract() {
        let t1: Tuple = [3., 2., 1., 1.];
        let t2: Tuple = [5., 6., 7., 1.];
        assert_eq!(is_equal(subtract(t1, t2), [-2., -4., -6., 0.]), true);

        let t3: Tuple = [3., 2., 1., 1.];
        let t4: Tuple = [5., 6., 7., 0.];
        assert_eq!(is_equal(subtract(t3, t4), [-2., -4., -6., 1.]), true);

        let t3: Tuple = [3., 2., 1., 0.];
        let t4: Tuple = [5., 6., 7., 0.];
        assert_eq!(is_equal(subtract(t3, t4), [-2., -4., -6., 0.]), true);
    }

    #[test]
    fn test_negate() {
        assert_eq!(is_equal(negate([1., -2., 3., -4.]), [-1., 2., -3., 4.]), true);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(is_equal(multiply([1., -2., 3., -4.], 3.5), [3.5, -7., 10.5, -14.]), true);
        assert_eq!(is_equal(multiply([1., -2., 3., -4.], 0.5), [0.5, -1., 1.5, -2.]), true);
    }

    #[test]
    fn test_divide() {
        assert_eq!(is_equal(divide([1., -2., 3., -4.], 2.), [0.5, -1., 1.5, -2.]), true);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(float::is_equal(magnitude([1., 0., 0., 0.]), 1.0), true);
        assert_eq!(float::is_equal(magnitude([0., 1., 0., 0.]), 1.0), true);
        assert_eq!(float::is_equal(magnitude([0., 0., 1., 0.]), 1.0), true);

        assert_eq!(float::is_equal(magnitude([1., 2., 3., 0.]), 14.0_f64.sqrt()), true);

        assert_eq!(float::is_equal(magnitude([-1., -2., -3., 0.]), 14.0_f64.sqrt()), true);
    }

    #[test]
    fn test_normalize() {
        assert_eq!(is_equal(normalize([4., 0., 0., 0.]), [1., 0., 0., 0.]), true);

        assert_eq!(is_equal(normalize([1., 2., 3., 0.]), [0.26726, 0.53452, 0.80178, 0.]), true);

        let n = normalize([1., 2., 3., 0.]);
        assert_eq!(float::is_equal(magnitude(n), 1.), true);
    }

    #[test]
    fn test_dot() {
        let t1: Tuple = [1., 2., 3., 0.];
        let t2: Tuple = [2., 3., 4., 0.];
        assert_eq!(float::is_equal(dot(t1, t2), 20.), true);
    }

    #[test]
    fn test_cross() {
        let t1: Tuple = [1., 2., 3., 0.];
        let t2: Tuple = [2., 3., 4., 0.];
        assert_eq!(is_equal(cross(t1, t2), [-1., 2., -1., 0.]), true);

        assert_eq!(is_equal(cross(t2, t1), [1., -2., 1., 0.]), true);
    }

    #[test]
    fn test_reflect_45_degrees() {
        let incident = vector(1., -1., 0.);
        let normal = vector(0., 1., 0.);
        let reflected = reflect(incident, normal);
        assert!(is_equal(reflected, vector(1., 1., 0.)));
    }

    #[test]
    fn test_reflect_slanted_surface() {
        let incident = vector(0., -1., 0.);
        let normal = vector(2_f64.sqrt()/2., 2_f64.sqrt()/2., 0.);
        let reflected = reflect(incident, normal);
        assert!(is_equal(reflected, vector(1., 0., 0.)));
    }
}
