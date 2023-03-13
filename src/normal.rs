use crate::{object::Object, tuples::Tuple, eq};


/// Reflects vector `v` in a surface with `normal`.
///
/// Assumes `normal` is a unit vector.
pub fn reflect(incoming: Tuple, normal: Tuple) -> Tuple {
    debug_assert!(incoming.dot(&normal) < 0.);
    incoming - normal * 2.0 * incoming.dot(&normal)
}


#[cfg(test)]
mod reflection_tests {
    use proptest::prelude::*;
    use crate::eq;
    use crate::tuples::proptest_strategies::vector;

    use super::*;

    proptest! {

        #[test]
        fn reflection_has_same_magnitude_as_incoming(
            normal in vector(100.).prop_map(|v| v.unit()),
            incoming in vector(100.),
        ) {
            prop_assume!(normal.dot(&incoming) < 0.);
            assert!(eq(
                reflect(incoming, normal).magnitude(),
                incoming.magnitude()
            ))
        }

        #[test]
        fn reflection_has_opposite_dot_with_normal_as_incoming(
            normal in vector(100.).prop_map(|v| v.unit()),
            incoming in vector(100.),
        ) {
            prop_assume!(normal.dot(&incoming) < 0.);
            assert!(eq(
                reflect(incoming, normal).dot(&normal),
                -incoming.dot(&normal)
            ))
        }

    }
}

/// Compute the refracted continuation of the incoming ray.
///
/// Assumes both input vectors are unit vectors.
/// the nfrom_over_nto is the ratio nf/nt
/// where nf is the refractive index of the old material
/// and nt is the refractive index of the new material.
pub fn refract(
    incoming: &Tuple,
    normal: Tuple,
    nfrom_over_nto: f64,
) -> Tuple {
    debug_assert!(eq(incoming.magnitude_squared(), 1.));
    debug_assert!(eq(normal.magnitude_squared(), 1.));
    debug_assert!(incoming.dot(&normal) < 0.);
    let cos_theta = -incoming.dot(&normal).min(1.);

    let out_perp = (incoming + normal * cos_theta) * nfrom_over_nto;
    let out_parallel = normal * -((1.-out_perp.magnitude_squared()).abs().sqrt());
    out_perp + out_parallel
}


pub fn normal_at_sphere(obj: &Object, world_point: &Tuple) -> Tuple {
    let object_point = &obj.inverse_transform / world_point;
    let object_normal = object_point - Tuple::point(0., 0., 0.);
    // The correct transformation for normals isn't what you expect!
    // (applying the inverse will squash normals,
    // preventing them from being perpendicular to the surface.)

    // Optimisation opportunities:
    // - only matmul the non-w parts
    // - skip creating a new matrix when transposing;
    //   instead, just redefine matmul / indexing.
    let mut normal = &obj.inverse_transform.transpose() / object_normal;
    // Our implementation of translations doesn't
    // play nice with transpose, but we can just zero out
    // w to ignore this, since we know normals should be vectors :)
    normal.w = 0.0;
    normal.unit()
}

#[cfg(test)]
mod sphere_normal_tests {
    use std::f64::consts::PI;

    use crate::transformations::{translation, rotation, Axis, Transformable};

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn normal_is_always_a_unit_vector(
            x in -1.0..1.0,
            y in -1.0..1.0,
            z in -1.0..1.0,
        ) {
            let s = Object::new_sphere();
            let n = s.normal_at(Tuple::point(x, y, z));
            assert_eq!(n, n.unit());
        }
    }

    #[test]
    fn normal_on_sphere_at_point_on_x_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Tuple::point(1., 0., 0.));
        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn normal_on_sphere_at_point_on_y_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Tuple::point(0., 1., 0.));
        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn normal_on_sphere_at_point_on_z_axis() {
        let s = Object::new_sphere();
        let n = s.normal_at(Tuple::point(0., 0., 1.));
        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let s = Object::new_sphere().with_transform(
            translation(0., 1., 0.)
        );
        assert_eq!(
            s.normal_at(Tuple::point(0., 1.70711, -0.70711)),
            Tuple::vector(0., 0.70711, -0.70711)
        )
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let s = Object::new_sphere().with_transform(
            rotation(Axis::Z, PI/5.0)
            .scale(1., 0.5, 1.)
        );
        assert_eq!(
            s.normal_at(Tuple::point(0., 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0)),
            Tuple::vector(0., 0.97014, -0.24254)
        )
    }

}