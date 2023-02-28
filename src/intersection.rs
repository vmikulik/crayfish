use crate::ray::Ray;
use crate::shapes::Sphere;

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