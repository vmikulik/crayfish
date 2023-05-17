use crate::{ray::Ray, object::Object, intersection::Intersection, tuples::{Tuple, Point, Vector}};


/// Returns the intersection(s) of a ray (in sphere-space) with a sphere.
///
/// By 'in sphere-space', we mean that we're using coordinates where
/// the sphere's origin is at the origin, and its radius is 1.0.
pub fn intersect_sphere<'a>(ray: &Ray, obj: &'a Object) -> Vec<Intersection<'a>>{

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

#[cfg(test)]
mod sphere_intersection_tests {
    use super::*;
    use crate::shapes::Shape;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::from_coords(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let s = Object::new(Shape::Sphere);
        let xs = intersect_sphere(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::from_coords(0.0, 1.0, -5.0, 0.0, 0.0, 1.0);
        let s = Object::new(Shape::Sphere);
        let xs = intersect_sphere(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::from_coords(0.0, 2.0, -5.0, 0.0, 0.0, 1.0);
        let s = Object::new(Shape::Sphere);
        let xs = intersect_sphere(&r, &s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_starts_inside_sphere() {
        let r = Ray::from_coords(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let s = Object::new(Shape::Sphere);
        let xs = intersect_sphere(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::from_coords(0.0, 0.0, 5.0, 0.0, 0.0, 1.0);
        let s = Object::new_sphere();
        let xs = intersect_sphere(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

}


pub fn normal_at_sphere(obj: &Object, world_point: &Tuple<Point>) -> Tuple<Vector> {
    let object_point = &obj.inverse_transform / world_point;
    let object_normal = object_point - Tuple::point(0., 0., 0.);
    // The correct transformation for normals isn't what you expect!
    // (applying the inverse will squash normals,
    // preventing them from being perpendicular to the surface.)

    let normal = &obj.inverse_transform_transposed / object_normal;
    normal.unit()
}


#[cfg(test)]
mod sphere_normal_tests {
    use std::f64::consts::PI;

    use crate::transformations::{translation, rotation, Axis, Transformable};

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn normal_is_always_a_unit_vector(
            x in -1.0..1.0,
            y in -1.0..1.0,
            z in -1.0..1.0,
        ) {
            let s = Object::new_sphere();
            let n = s.normal_at(Tuple::point(x, y, z));
            assert_eq!(n, n.unit());
        }
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Tuple::point(1., 0., 0.));
        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Tuple::point(0., 1., 0.));
        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Tuple::point(0., 0., 1.));
        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let s = Object::new_sphere().with_transform(
            translation(0., 1., 0.)
        );
        assert_eq!(
            s.normal_at(Tuple::point(0., 1.70711, -0.70711)),
            Tuple::vector(0., 0.70711, -0.70711)
        )
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let s = Object::new_sphere().with_transform(
            rotation(Axis::Z, PI/5.0)
            .scale(1., 0.5, 1.)
        );
        assert_eq!(
            s.normal_at(Tuple::point(0., 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0)),
            Tuple::vector(0., 0.97014, -0.24254)
        )
    }

}