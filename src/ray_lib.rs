use crate::tuples_lib::Tuple;
use crate::shapes::Sphere;


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

    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &self.direction * t
    }

}

pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl Intersection<'_> {
    pub fn new<'a>(t: f64, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
}

pub fn intersect<'a, T: Intersectable>(ray: &Ray, obj: &'a T) -> Vec<Intersection<'a>> {
    obj.intersect(ray)
}