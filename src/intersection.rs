use crate::{ray::Ray, object::Object, object::Shape, minimum_by_key, eq};

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


/// Returns the intersection(s) of a ray (in sphere-space) with a sphere.
///
/// By 'in sphere-space', we mean that we're using coordinates where
/// the sphere's origin is at the origin, and its radius is 1.0.
fn intersect_sphere<'a>(ray: &Ray, obj: &'a Object) -> Vec<Intersection<'a>>{

    let sphere_to_ray = &ray.origin - crate::tuples::Tuple::point(0.0, 0.0, 0.0);
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return vec![]
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
    return vec![
        Intersection::new(t1, obj),
        Intersection::new(t2, obj),
    ]
}

/// Returns the intersection(s) of a ray (in cube-space) with a cube.
fn intersect_cube<'a>(ray: &Ray, obj: &'a Object) -> Vec<Intersection<'a>>{

    let o = ray.origin.as_array();
    let d = ray.direction.as_array();

    let mut intersections: Vec<Intersection> = vec![];

    let intersection_on_face = |t: f64, dir: usize| {
        for other_dir in 0..3 {
            if other_dir == dir {
                continue;
            }
            if (o[other_dir] + t * d[other_dir]).abs() > 1.0 {
                return false
            }
        }
        true
    };

    for dir in [0, 1, 2] {
        if eq(d[dir], 0.0) {
            continue
        }
        for face in [-1.0, 1.0] {
            // find the intersection of the ray with the plane
            let t = (face - o[dir]) / d[dir];

            if (t < 0.0) | !intersection_on_face(t, dir) {
                // intersection is behind the ray or not on the cube's face
                continue
            }

            intersections.push(Intersection::new(t, obj));
            if intersections.len() == 2 {
                return intersections
            }
        }
    }

    return intersections

}

#[cfg(test)]
mod sphere_intersection_tests {
    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::from_coords(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::from_coords(0.0, 1.0, -5.0, 0.0, 0.0, 1.0);
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::from_coords(0.0, 2.0, -5.0, 0.0, 0.0, 1.0);
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_starts_inside_sphere() {
        let r = Ray::from_coords(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::from_coords(0.0, 0.0, 5.0, 0.0, 0.0, 1.0);
        let s = Object::new_sphere();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

}

#[cfg(test)]
mod cube_intersection_tests {

    use super::*;
    use proptest::prelude::*;
    use crate::tuples::proptest_strategies;

    #[test]
    fn ray_intersects_cube_at_two_points() {
        let r = Ray::from_coords(-10.0, 0., 0., 1., 0., 0.);
        let s = Object::new(Shape::Cube);
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 9.0);
        assert_eq!(xs[1].t, 11.0);
    }

    #[test]
    fn ray_misses_cube() {
        let r = Ray::from_coords(-10.0, 0., 0., -1., 0., 0.);
        let s = Object::new(Shape::Cube);
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_from_inside_cube() {
        let r = Ray::from_coords(0., 0., 0., 0., 0., 1.);
        let s = Object::new(Shape::Cube);
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
    }

    proptest! {
        #[test]
        fn ray_aiming_inside_cube_always_hits_twice(
            origin in proptest_strategies::point(100.),
            target in proptest_strategies::point(1.),
        ) {
            prop_assume!(origin.magnitude() > 3.0_f64.sqrt());
            let r = Ray::new(origin, target - origin);
            let s = Object::new(Shape::Cube);
            let xs = intersect(&r, &s);
            prop_assert!(xs.len() == 2);
        }
    }
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