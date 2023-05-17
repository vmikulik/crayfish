use crate::materials::Material;
use crate::matrix::Matrix;
use crate::tuples::{Tuple, Point, Vector};
use crate::shapes::Shape;

#[derive(Debug)]
pub struct Object {
    pub shape: Shape,
    pub material: Box<dyn Material>,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub inverse_transform_transposed: Matrix,
}

impl Object {
    pub fn new(shape: Shape) -> Object {
        Object {
            shape,
            material: Default::default(),
            transform: Matrix::identity(4),
            inverse_transform: Matrix::identity(4),
            inverse_transform_transposed: Matrix::identity(4),
        }
    }

    pub fn new_sphere() -> Object {
        Object::new(Shape::Sphere)
    }

    pub fn with_material(self, material: Box<dyn Material>) -> Object {
        Object {
            material,
            ..self
        }
    }

    pub fn with_transform(self, transform: Matrix) -> Object {
        let inverse_transform = transform.inverse().unwrap();
        let inverse_transform_transposed = inverse_transform.transpose();
        Object {
            transform,
            inverse_transform,
            inverse_transform_transposed,
            ..self
        }
    }

    pub fn normal_at(&self, position: Tuple<Point>) -> Tuple<Vector> {
        self.shape.normal_at(&self, &position)
    }
}


#[cfg(test)]
mod object_transform_tests {
    use super::*;
    use crate::transformations::translation;

    #[test]
    fn a_spheres_default_transformation() {
        let s = Object::new_sphere();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let s = Object::new_sphere()
            .with_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(s.transform, translation(2.0, 3.0, 4.0));
    }
}