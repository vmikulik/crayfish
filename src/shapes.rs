use crate::{tuples::{Tuple, Point, Vector}, object::Object};

pub mod sphere;
pub mod cube;

#[derive(Debug)]
pub enum Shape {
    Sphere,
    Cube,
}

impl Shape {
    pub fn normal_at(&self, obj: &Object, position: &Tuple<Point>) -> Tuple<Vector> {
        match self {
            Shape::Sphere => sphere::normal_at_sphere(obj, position),
            Shape::Cube => cube::normal_at_cube(obj, position),
        }
    }
}