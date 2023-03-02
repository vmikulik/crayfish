use crate::matrix::Matrix;
use crate::tuples::Tuple;
use crate::normal::{
    normal_at_sphere
};

#[derive(Debug)]
pub enum Shape {
    Sphere,
}

#[derive(Debug)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix,
    pub inverse_transform: Matrix,
}

impl Object {
    pub fn new_sphere() -> Object {
        Object {
            shape: Shape::Sphere,
            transform: Matrix::identity(4),
            inverse_transform: Matrix::identity(4),
        }
    }

    pub fn with_transform(self, transform: Matrix) -> Object {
        let inverse_transform = transform.inverse().unwrap();
        Object {
            transform,
            inverse_transform,
            ..self
        }
    }

    pub fn normal_at(self, position: Tuple) -> Tuple {
        match self.shape {
            Shape::Sphere => normal_at_sphere(&self, position)
        }
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