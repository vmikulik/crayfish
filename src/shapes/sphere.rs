use crate::{ray::Ray, object::Object, intersection::Intersection};


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
