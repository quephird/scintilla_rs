use crate::shape::Shape;
use crate::sphere;

#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: &dyn Shape) -> Intersection {
        Intersection {
            t: t,
            object: object,
        }
    }
}

pub fn hit<'a>(intersections: &'a mut Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    intersections.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap());
    intersections
        .iter()
        .filter(|i| i.t >= 0.)
        .nth(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_all_positive_t() {
        let s = sphere::Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let mut intersections = vec![i1.clone(), i2.clone()];
        let hit = hit(&mut intersections).unwrap();
        assert_eq!(hit.t, i1.t);
    }

    #[test]
    fn test_hit_mixture() {
        let s = sphere::Sphere::new();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let mut intersections = vec![i1.clone(), i2.clone()];
        let hit = hit(&mut intersections).unwrap();
        assert_eq!(hit.t, i2.t);
    }

    #[test]
    fn test_hit_all_negative_t() {
        let s = sphere::Sphere::new();
        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let mut intersections = vec![i1.clone(), i2.clone()];
        let hit = hit(&mut intersections);
        assert!(hit.is_none());
    }

    #[test]
    fn test_hit_unsorted_intersection() {
        let s = sphere::Sphere::new();
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let mut intersections = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];
        let hit = hit(&mut intersections).unwrap();
        assert_eq!(hit.t, i4.t);
    }
}
