use crate::{tuples::{Tuple, Vector}, eq};


/// Reflects vector `v` in a surface with `normal`.
///
/// Assumes `normal` is a unit vector.
pub fn reflect(incoming: &Tuple<Vector>, normal: &Tuple<Vector>) -> Tuple<Vector> {
    incoming - normal * 2.0 * incoming.dot(normal)
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
                reflect(&incoming, &normal).magnitude(),
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
                reflect(&incoming, &normal).dot(&normal),
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
    incoming: &Tuple<Vector>,
    normal: Tuple<Vector>,
    nfrom_over_nto: f64,
) -> Tuple<Vector> {
    debug_assert!(eq(incoming.magnitude_squared(), 1.));
    debug_assert!(eq(normal.magnitude_squared(), 1.));
    debug_assert!(incoming.dot(&normal) < 0.);
    let cos_theta = -incoming.dot(&normal).min(1.);

    let out_perp = (incoming + normal * cos_theta) * nfrom_over_nto;
    let out_parallel = normal * -((1.-out_perp.magnitude_squared()).abs().sqrt());
    out_perp + out_parallel
}
