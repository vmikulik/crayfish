use crate::matrix::Matrix;


pub enum Axis {
    X,
    Y,
    Z,
}

pub trait Transformable {
    fn rotate(&self, axis: Axis, radians: f64) -> Self;
    fn translate(&self, x: f64, y: f64, z: f64) -> Self;
    fn scale(&self, x: f64, y: f64, z: f64) -> Self;
}

impl Transformable for Matrix {
    fn rotate(&self, axis: Axis, radians: f64) -> Matrix {
        rotation(axis, radians) / self
    }

    fn translate(&self, x: f64, y: f64, z: f64) -> Matrix {
        translation(x, y, z) / self
    }

    fn scale(&self, x: f64, y: f64, z: f64) -> Matrix {
        scaling(x, y, z) / self
    }
}


/// Translates points by the given x, y, and z values.
///
/// Does not affect vectors.
pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0,3)] = x;
    m[(1,3)] = y;
    m[(2,3)] = z;
    m
}

/// Scales Tuples by the given x, y, and z values.
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0,0)] = x;
    m[(1,1)] = y;
    m[(2,2)] = z;
    m
}

/// Rotates Tuples around the given axis by the given radians.
pub fn rotation(axis: Axis, radians: f64) -> Matrix {
    let mut m = Matrix::identity(4);
    match axis {
        Axis::X => {
            m[(1,1)] = radians.cos();
            m[(1,2)] = -radians.sin();
            m[(2,1)] = radians.sin();
            m[(2,2)] = radians.cos();
        }
        Axis::Y => {
            m[(0,0)] = radians.cos();
            m[(0,2)] = radians.sin();
            m[(2,0)] = -radians.sin();
            m[(2,2)] = radians.cos();
        }
        Axis::Z => {
            m[(0,0)] = radians.cos();
            m[(0,1)] = -radians.sin();
            m[(1,0)] = radians.sin();
            m[(1,1)] = radians.cos();
        }
    }
    m
}

/// Shears Tuples.
///
/// xy is the amount of shear along x in proportion to y.
/// the other parameters are similar.
pub fn shearing(
    xy: f64,
    xz: f64,
    yx: f64,
    yz: f64,
    zx: f64, 
    zy: f64,
) -> Matrix {
    let mut m = Matrix::identity(4);
    m[(0,1)] = xy;
    m[(0,2)] = xz;
    m[(1,0)] = yx;
    m[(1,2)] = yz;
    m[(2,0)] = zx;
    m[(2,1)] = zy;
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Axis::{X, Y, Z};
    use crate::tuples::*;
    use crate::Result;

    #[test]
    fn translating_points_moves_them() -> Result<()> {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform.matmul_t(&p)?, Tuple::point(2.0, 1.0, 7.0));
        Ok(())
    }

    #[test]
    fn translating_by_inverse_moves_back() -> Result<()> {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse()?;
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv.matmul_t(&p)?, Tuple::point(-8.0, 7.0, 3.0));
        Ok(())
    }

    #[test]
    fn translating_vectors_does_nothing() -> Result<()> {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform.matmul_t(&v)?, v);
        Ok(())
    }


    #[test]
    fn scaling_vectors() -> Result<()> {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform.matmul_t(&v)?, Tuple::vector(-8.0, 18.0, 32.0));
        Ok(())
    }


    #[test]
    fn scaling_points() -> Result<()> {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform.matmul_t(&p)?, Tuple::point(-8.0, 18.0, 32.0));
        Ok(())
    }


    #[test]
    fn rotating_points_around_x_axis() -> Result<()> {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation(X, std::f64::consts::PI / 4.0);
        let full_quarter = rotation(X, std::f64::consts::PI / 2.0);
        assert_eq!(
            half_quarter.matmul_t(&p)?,
            Tuple::point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0)
        );
        assert_eq!(
            full_quarter.matmul_t(&p)?,
            Tuple::point(0.0, 0.0, 1.0)
        );
        Ok(())
    }


    #[test]
    fn rotating_points_around_y_axis() -> Result<()> {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation(Y, std::f64::consts::PI / 4.0);
        let full_quarter = rotation(Y, std::f64::consts::PI / 2.0);
        assert_eq!(
            half_quarter.matmul_t(&p)?,
            Tuple::point(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.0)
        );
        assert_eq!(
            full_quarter.matmul_t(&p)?,
            Tuple::point(1.0, 0.0, 0.0)
        );
        Ok(())
    }

    #[test]
    fn rotating_points_around_z_axis() -> Result<()> {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation(Z, std::f64::consts::PI / 4.0);
        let full_quarter = rotation(Z, std::f64::consts::PI / 2.0);
        assert_eq!(
            half_quarter.matmul_t(&p)?,
            Tuple::point(-(2f64.sqrt()) / 2.0, 2f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(
            full_quarter.matmul_t(&p)?,
            Tuple::point(-1.0, 0.0, 0.0)
        );
        Ok(())
    }


    #[test]
    fn fluent_api() -> Result<()> {
        let p = Tuple::point(1.0, 0., 0.);
        let transform = Matrix::identity(4)
            .rotate(Z, std::f64::consts::PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(
            transform / &p,
            Tuple::point(10.0, 10.0, 7.0)
        );
        Ok(())
    }

}