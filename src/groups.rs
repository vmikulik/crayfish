use crate::{object::Object, intersection::{Intersectable, Intersection, intersect}, ray::Ray};

pub struct ObjectGroup {
    objects: Vec<Object>
}

impl ObjectGroup {
    pub fn new() -> ObjectGroup {
        ObjectGroup {objects: vec![]}
    }

    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj)
    }
}

impl Intersectable for ObjectGroup {
    fn intersect<'a>(ray: &Ray, obj: &'a Self) -> Vec<Intersection<'a>> {
        let mut intersections = vec![];
        for inner_obj in obj.objects.iter() {
            intersections.append(&mut intersect(&ray, inner_obj));
        }
        intersections
    }
}