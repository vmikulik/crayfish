use crate::{
    ray::Ray,
    object::Object,
    shapes::Shape,
    shapes::sphere::intersect_sphere,
    shapes::cube::intersect_cube,
    minimum_by_key,
};

#[derive(Debug, Clone, Copy)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

impl Intersection<'_> {
    pub fn new<'a>(t: f64, object: & Object) -> Intersection<'_> {
        Intersection { t, object }
    }
}

pub trait Intersectable {
    fn intersect<'a>(ray: &Ray, obj: &'a Self) -> Vec<Intersection<'a>>;
}


impl Intersectable for Object {
    fn intersect<'a>(ray: &Ray, obj: &'a Self) -> Vec<Intersection<'a>> {
        let ray_in_sphere_space = ray.transform(&obj.inverse_transform);
        match &obj.shape {
            Shape::Sphere => intersect_sphere(&ray_in_sphere_space, obj),
            Shape::Cube => intersect_cube(&ray_in_sphere_space, obj),
        }
    }
}


pub fn intersect<'a>(ray: &Ray, obj: &'a impl Intersectable) ->  Vec<Intersection<'a>> {
    Intersectable::intersect(ray, obj)
}


pub fn hit<'a, 'b>(
    xs: &'b [Intersection<'a>],
    min_t: f64,
) -> Option<&'b Intersection<'a>> {
    let positive_hits = xs
        .iter()
        .filter(|x| min_t < x.t);

    minimum_by_key(positive_hits, |x| x.t)
}

#[cfg(test)]
mod hit_tests {
    use super::*;

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        let i = hit(&xs, 0.).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2];
        let i = hit(&xs, 0.).unwrap();
        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];
        let i = hit(&xs, 0.);
        assert!(i.is_none());
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Object::new_sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs, 0.).unwrap();
        assert_eq!(i.t, 2.0);
    }

}