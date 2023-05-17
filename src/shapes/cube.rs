use crate::{ray::Ray, object::Object, intersection::Intersection, eq};


/// Returns the intersection(s) of a ray (in cube-space) with a cube.
pub fn intersect_cube<'a>(ray: &Ray, obj: &'a Object) -> Vec<Intersection<'a>>{

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
mod cube_intersection_tests {

    use super::*;
    use proptest::prelude::*;
    use crate::tuples::proptest_strategies;
    use crate::shapes::Shape;

    #[test]
    fn ray_intersects_cube_at_two_points() {
        let r = Ray::from_coords(-10.0, 0., 0., 1., 0., 0.);
        let s = Object::new(Shape::Cube);
        let xs = intersect_cube(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 9.0);
        assert_eq!(xs[1].t, 11.0);
    }

    #[test]
    fn ray_misses_cube() {
        let r = Ray::from_coords(-10.0, 0., 0., -1., 0., 0.);
        let s = Object::new(Shape::Cube);
        let xs = intersect_cube(&r, &s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_from_inside_cube() {
        let r = Ray::from_coords(0., 0., 0., 0., 0., 1.);
        let s = Object::new(Shape::Cube);
        let xs = intersect_cube(&r, &s);
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
            let xs = intersect_cube(&r, &s);
            prop_assert!(xs.len() == 2);
        }
    }
}