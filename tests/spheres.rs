#[cfg(test)]
mod tests {
    use crayfish::object::Object;
    use crayfish::ray::Ray;
    use crayfish::transformations::{scaling, translation};
    use crayfish::intersection::intersect;

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::from_coords(0., 0., -5., 0., 0., 1.);
        let s = Object::new_sphere()
            .with_transform(scaling(2., 2., 2.));
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::from_coords(0., 0., -5., 0., 0., 1.);
        let s = Object::new_sphere()
            .with_transform(translation(5., 0., 0.));
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 0);
    }

}