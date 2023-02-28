#[derive(Debug)]
pub enum Shape {
    Sphere,
}

#[derive(Debug)]
pub struct Object {
    pub shape: Shape,
}

impl Object {
    pub fn new_sphere() -> Object {
        Object {shape: Shape::Sphere}
    }
}