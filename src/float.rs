pub(crate) const EPSILON: f64 = 0.00001;

pub fn is_equal(a: f64, b: f64) -> bool {
    if (a - b).abs() < EPSILON {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_equal() {
        assert_eq!(is_equal(0., 0.), true);
        assert_eq!(is_equal(0., 0.000005), true);

        assert_eq!(is_equal(0., 1.), false);
        assert_eq!(is_equal(0., 0.00001), false);
    }
}