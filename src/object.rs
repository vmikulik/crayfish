use crate::matrix::Matrix;

#[derive(Debug)]
pub enum Shape {
    Sphere,
}

#[derive(Debug)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix,
}

impl Object {
    pub fn new_sphere() -> Object {
        Object {
            shape: Shape::Sphere,
            transform: Matrix::identity(4),
        }
    }

    pub fn with_transform(self, transform: Matrix) -> Object {
        Object {
            transform,
            ..self
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