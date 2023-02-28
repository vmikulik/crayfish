use crate::tuples::Tuple;
use crate::matrix::Matrix;


pub struct Ray {
    pub origin: Tuple,    // Point
    pub direction: Tuple, // Vector
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn from_coords(x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> Ray {
        Ray {
            origin: Tuple::point(x, y, z),
            direction: Tuple::vector(dx, dy, dz),
        }
    }

    pub fn transform(&self, m: &Matrix) -> Ray {
        Ray {
            origin: m / &self.origin,
            direction: m / &self.direction,
        }
    }

    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &self.direction * t
    }

}


#[cfg(test)]
mod ray_transform_tests {
    use super::*;
    use crate::transformations::*;

    #[test]
    fn translating_a_ray() {
        let r = Ray::from_coords(1.0, 2.0, 3.0, 0.0, 1.0, 0.0);
        let m = translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(4., 6., 8.));
        assert_eq!(r2.direction, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::from_coords(1., 2., 3., 0., 1., 0.,);
        let m = scaling(2., 3., 4.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(2., 6., 12.));
        assert_eq!(r2.direction, Tuple::vector(0., 3., 0.));
    }
}

