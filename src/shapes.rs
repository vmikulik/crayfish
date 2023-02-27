use crate::ray_lib::Intersection;
use crate::ray_lib::Intersectable;
use crate::ray_lib::intersect;
use crate::ray_lib::Ray;

pub struct Sphere {}

impl Sphere {
    pub fn unit_at_origin() -> Sphere {
        Sphere {}
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>{

        let sphere_to_ray = &ray.origin - crate::tuples_lib::Tuple::point(0.0, 0.0, 0.0);
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
            Intersection::new(t1, self),
            Intersection::new(t2, self),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::from_coords(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let s = Sphere::unit_at_origin();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::from_coords(0.0, 1.0, -5.0, 0.0, 0.0, 1.0);
        let s = Sphere::unit_at_origin();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::from_coords(0.0, 2.0, -5.0, 0.0, 0.0, 1.0);
        let s = Sphere::unit_at_origin();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_starts_inside_sphere() {
        let r = Ray::from_coords(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let s = Sphere::unit_at_origin();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::from_coords(0.0, 0.0, 5.0, 0.0, 0.0, 1.0);
        let s = Sphere::unit_at_origin();
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

}