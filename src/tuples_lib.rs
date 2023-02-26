use crate::constants::EPSILON;


#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}


impl std::cmp::PartialEq for Tuple {
    fn eq(&self, _rhs: &Tuple) -> bool {
        (self.x - _rhs.x).abs() < EPSILON
        && (self.y - _rhs.y).abs() < EPSILON
        && (self.z - _rhs.z).abs() < EPSILON
        && (self.w - _rhs.w).abs() < EPSILON
    }
}

impl std::ops::Add<&Tuple> for &Tuple {
    type Output = Tuple;

    fn add(self, _rhs: &Tuple) -> Tuple {
        Tuple {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}
impl std::ops::Add<Tuple> for &Tuple {
    type Output = Tuple;
    fn add(self, _rhs: Tuple) -> Tuple {
        self + &_rhs
    }
}
impl std::ops::Add<&Tuple> for Tuple {
    type Output = Tuple;
    fn add(self, _rhs: &Tuple) -> Tuple {
        &self + _rhs
    }
}
impl std::ops::Add<Tuple> for Tuple {
    type Output = Tuple;
    fn add(self, _rhs: Tuple) -> Tuple {
        &self + &_rhs
    }
}



impl std::ops::Sub<&Tuple> for &Tuple {
    type Output = Tuple;

    fn sub(self, _rhs: &Tuple) -> Tuple {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}
impl std::ops::Sub<Tuple> for &Tuple {
    type Output = Tuple;
    fn sub(self, _rhs: Tuple) -> Tuple {
        self - &_rhs
    }
}
impl std::ops::Sub<&Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, _rhs: &Tuple) -> Tuple {
        &self - _rhs
    }
}
impl std::ops::Sub<Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, _rhs: Tuple) -> Tuple {
        &self - &_rhs
    }
}


impl std::ops::Neg for &Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl std::ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        -&self
    }
}


impl std::ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, _rhs: f64) -> Tuple {
        Tuple {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs,
        }
    }
}


impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, _rhs: f64) -> Tuple {
        &self * _rhs
    }
}


impl std::ops::Div<f64> for &Tuple {
    type Output = Tuple;

    fn div(self, _rhs: f64) -> Tuple {
        Tuple {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            w: self.w / _rhs,
        }
    }
}


impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, _rhs: f64) -> Tuple {
        &self / _rhs
    }
}


impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Tuple {
        self / self.magnitude()
    }

    pub fn cross(&self, _rhs: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * _rhs.z - self.z * _rhs.y,
            self.z * _rhs.x - self.x * _rhs.z,
            self.x * _rhs.y - self.y * _rhs.x,
        )
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w1_is_a_point() {
        let a  = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(a.is_vector());
        assert!(!a.is_point());
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
    }

    #[test]
    fn vector_constructor_returns_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        let b = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(a, b);
    }

    #[test]
    fn point_constructor_returns_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let b = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(a, b);
    }

    #[test]
    fn adding_vectors() {
        let a = Tuple::vector(3., -2., 5.);
        let b = Tuple::vector(-2., 3., 1.);
        let c = Tuple::vector(1., 1., 6.);
        assert_eq!(a + b, c);
    }

    #[test]
    fn subtracting_points_returns_vector() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::point(5., 6., 7.);
        let c = Tuple::vector(-2., -4., -6.);
        assert_eq!(a - b, c);
    }

    #[test]
    fn subtracting_vector_from_point_returns_point() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::vector(5., 6., 7.);
        let c = Tuple::point(-2., -4., -6.);
        assert_eq!(a - b, c);
    }

    #[test]
    fn subtracting_vector_from_vector_returns_vector() {
        let a = Tuple::vector(3., 2., 1.);
        let b = Tuple::vector(5., 6., 7.);
        let c = Tuple::vector(-2., -4., -6.);
        assert_eq!(a - b, c);
    }

    #[test]
    fn negating_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        let b = Tuple::new(-1., 2., -3., 4.);
        assert_eq!(-a, b);
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let b = Tuple::new(3.5, -7., 10.5, -14.);
        assert_eq!(a * 3.5, b);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let b = Tuple::new(0.5, -1., 1.5, -2.);
        assert_eq!(a / 2., b);
    }

    #[test]
    fn magnitude_of_vector() {
        let a = Tuple::vector(1., 0., 0.);
        let b = Tuple::vector(0., 1., 0.);
        let c = Tuple::vector(0., 0., 1.);
        let d = Tuple::vector(1., 2., 3.);
        let e = Tuple::vector(-1., -2., -3.);
        assert_eq!(a.magnitude(), 1.);
        assert_eq!(b.magnitude(), 1.);
        assert_eq!(c.magnitude(), 1.);
        assert_eq!(d.magnitude(), (14_f64).sqrt());
        assert_eq!(e.magnitude(), (14_f64).sqrt());
    }

    #[test]
    fn normalizing_vector() {
        let a = Tuple::vector(4., 0., 0.);
        let b = Tuple::vector(1., 2., 3.);
        assert_eq!(a.unit(), Tuple::vector(1., 0., 0.));
        assert_eq!(b.unit(), Tuple::vector(0.26726, 0.53452, 0.80178));
    }


    #[test]
    fn taking_cross_product() {
        let a = Tuple::vector(1., 2., 3.);
        let b = Tuple::vector(2., 3., 4.);
        assert_eq!(a.cross(&b), Tuple::vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), Tuple::vector(1., -2., 1.));
    }

}

