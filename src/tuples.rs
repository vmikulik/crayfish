use crate::constants::EPSILON;


/// This trait only exists to permit Tuple::new()
/// to be generic. It seems very hacky, and there must
/// be a better way, but it is what it is.
pub trait TupleType<K> {
    fn make_tuple(x: f64, y: f64, z: f64) -> Tuple<K>;
    fn as_array(t: &Tuple<K>) -> [f64; 4];
}

#[derive(Debug, Clone, Copy)]
pub struct Vector();

impl TupleType<Vector> for Vector {
    fn make_tuple(x: f64, y: f64, z: f64) -> Tuple<Vector> {
        Tuple::vector(x, y, z)
    }

    fn as_array(t: &Tuple<Vector>) -> [f64; 4] {
        [t.x, t.y, t.z, 0.0]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point();

impl TupleType<Point> for Point {
    fn make_tuple(x: f64, y: f64, z: f64) -> Tuple<Point> {
        Tuple::point(x, y, z)
    }

    fn as_array(t: &Tuple<Point>) -> [f64; 4] {
        [t.x, t.y, t.z, 1.0]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple<Kind> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    kind: std::marker::PhantomData<Kind>,
}

#[cfg(test)]
pub mod proptest_strategies {
    use proptest::prelude::*;
    use super::Tuple;
    use super::Vector;
    use super::Point;

    /// Strategy for vectors with given maximum values.
    pub fn vector(max_val: f64) -> impl Strategy<Value = Tuple<Vector>> {
        (
            -max_val..max_val,
            -max_val..max_val,
            -max_val..max_val,
        ).prop_map(|(x, y, z)| {
            Tuple::vector(x, y, z)
        })
    }

    /// Strategy for points with given maximum values.
    pub fn point(max_val: f64) -> impl Strategy<Value = Tuple<Point>> {
        (
            -max_val..max_val,
            -max_val..max_val,
            -max_val..max_val,
        ).prop_map(|(x, y, z)| {
            Tuple::point(x, y, z)
        })
    }
}

impl<K> std::cmp::PartialEq for Tuple<K> {
    fn eq(&self, _rhs: &Tuple<K>) -> bool {
        (self.x - _rhs.x).abs() < EPSILON
        && (self.y - _rhs.y).abs() < EPSILON
        && (self.z - _rhs.z).abs() < EPSILON
    }
}

// Vector + Vector = Vector
impl std::ops::Add<&Tuple<Vector>> for &Tuple<Vector> {
    type Output = Tuple<Vector>;

    fn add(self, _rhs: &Tuple<Vector>) -> Tuple<Vector> {
        Tuple {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            kind: std::marker::PhantomData::<Vector>,
        }
    }
}
impl std::ops::Add<Tuple<Vector>> for &Tuple<Vector> {
    type Output = Tuple<Vector>;
    fn add(self, _rhs: Tuple<Vector>) -> Tuple<Vector> {
        self + &_rhs
    }
}
impl std::ops::Add<&Tuple<Vector>> for Tuple<Vector> {
    type Output = Tuple<Vector>;
    fn add(self, _rhs: &Tuple<Vector>) -> Tuple<Vector> {
        &self + _rhs
    }
}
impl std::ops::Add<Tuple<Vector>> for Tuple<Vector> {
    type Output = Tuple<Vector>;
    fn add(self, _rhs: Tuple<Vector>) -> Tuple<Vector> {
        &self + &_rhs
    }
}


// Point + Vector = Point
impl std::ops::Add<&Tuple<Vector>> for &Tuple<Point> {
    type Output = Tuple<Point>;

    fn add(self, _rhs: &Tuple<Vector>) -> Tuple<Point> {
        Tuple {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            kind: std::marker::PhantomData::<Point>,
        }
    }
}
impl std::ops::Add<Tuple<Vector>> for &Tuple<Point> {
    type Output = Tuple<Point>;
    fn add(self, _rhs: Tuple<Vector>) -> Tuple<Point> {
        self + &_rhs
    }
}
impl std::ops::Add<&Tuple<Vector>> for Tuple<Point> {
    type Output = Tuple<Point>;
    fn add(self, _rhs: &Tuple<Vector>) -> Tuple<Point> {
        &self + _rhs
    }
}
impl std::ops::Add<Tuple<Vector>> for Tuple<Point> {
    type Output = Tuple<Point>;
    fn add(self, _rhs: Tuple<Vector>) -> Tuple<Point> {
        &self + &_rhs
    }
}

// Vector + Point = Point
impl std::ops::Add<&Tuple<Point>> for &Tuple<Vector> {
    type Output = Tuple<Point>;

    fn add(self, _rhs: &Tuple<Point>) -> Tuple<Point> {
        Tuple {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            kind: std::marker::PhantomData::<Point>,
        }
    }
}
impl std::ops::Add<Tuple<Point>> for &Tuple<Vector> {
    type Output = Tuple<Point>;
    fn add(self, _rhs: Tuple<Point>) -> Tuple<Point> {
        self + &_rhs
    }
}
impl std::ops::Add<&Tuple<Point>> for Tuple<Vector> {
    type Output = Tuple<Point>;
    fn add(self, _rhs: &Tuple<Point>) -> Tuple<Point> {
        &self + _rhs
    }
}
impl std::ops::Add<Tuple<Point>> for Tuple<Vector> {
    type Output = Tuple<Point>;
    fn add(self, _rhs: Tuple<Point>) -> Tuple<Point> {
        &self + &_rhs
    }
}

// Vector - Vector = Vector
impl std::ops::Sub<&Tuple<Vector>> for &Tuple<Vector> {
    type Output = Tuple<Vector>;

    fn sub(self, _rhs: &Tuple<Vector>) -> Tuple<Vector> {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            kind: std::marker::PhantomData::<Vector>,
        }
    }
}
impl std::ops::Sub<Tuple<Vector>> for &Tuple<Vector> {
    type Output = Tuple<Vector>;
    fn sub(self, _rhs: Tuple<Vector>) -> Tuple<Vector> {
        self - &_rhs
    }
}
impl std::ops::Sub<&Tuple<Vector>> for Tuple<Vector> {
    type Output = Tuple<Vector>;
    fn sub(self, _rhs: &Tuple<Vector>) -> Tuple<Vector> {
        &self - _rhs
    }
}
impl std::ops::Sub<Tuple<Vector>> for Tuple<Vector> {
    type Output = Tuple<Vector>;
    fn sub(self, _rhs: Tuple<Vector>) -> Tuple<Vector> {
        &self - &_rhs
    }
}

// Point - Point = Vector
impl std::ops::Sub<&Tuple<Point>> for &Tuple<Point> {
    type Output = Tuple<Vector>;

    fn sub(self, _rhs: &Tuple<Point>) -> Tuple<Vector> {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            kind: std::marker::PhantomData::<Vector>,
        }
    }
}
impl std::ops::Sub<Tuple<Point>> for &Tuple<Point> {
    type Output = Tuple<Vector>;
    fn sub(self, _rhs: Tuple<Point>) -> Tuple<Vector> {
        self - &_rhs
    }
}
impl std::ops::Sub<&Tuple<Point>> for Tuple<Point> {
    type Output = Tuple<Vector>;
    fn sub(self, _rhs: &Tuple<Point>) -> Tuple<Vector> {
        &self - _rhs
    }
}
impl std::ops::Sub<Tuple<Point>> for Tuple<Point> {
    type Output = Tuple<Vector>;
    fn sub(self, _rhs: Tuple<Point>) -> Tuple<Vector> {
        &self - &_rhs
    }
}

// Point - Vector = Point
impl std::ops::Sub<&Tuple<Vector>> for &Tuple<Point> {
    type Output = Tuple<Point>;

    fn sub(self, _rhs: &Tuple<Vector>) -> Tuple<Point> {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            kind: std::marker::PhantomData::<Point>,
        }
    }
}
impl std::ops::Sub<Tuple<Vector>> for &Tuple<Point> {
    type Output = Tuple<Point>;
    fn sub(self, _rhs: Tuple<Vector>) -> Tuple<Point> {
        self - &_rhs
    }
}
impl std::ops::Sub<&Tuple<Vector>> for Tuple<Point> {
    type Output = Tuple<Point>;
    fn sub(self, _rhs: &Tuple<Vector>) -> Tuple<Point> {
        &self - _rhs
    }
}
impl std::ops::Sub<Tuple<Vector>> for Tuple<Point> {
    type Output = Tuple<Point>;
    fn sub(self, _rhs: Tuple<Vector>) -> Tuple<Point> {
        &self - &_rhs
    }
}

impl<K> std::ops::Neg for &Tuple<K> {
    type Output = Tuple<K>;

    fn neg(self) -> Tuple<K> {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            kind: std::marker::PhantomData::<K>,
        }
    }
}
impl<K> std::ops::Neg for Tuple<K> {
    type Output = Tuple<K>;
    fn neg(self) -> Tuple<K> {
        -&self
    }
}


impl<K> std::ops::Mul<f64> for &Tuple<K> {
    type Output = Tuple<K>;

    fn mul(self, _rhs: f64) -> Tuple<K> {
        Tuple {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            kind: std::marker::PhantomData::<K>,
        }
    }
}


impl<K> std::ops::Mul<f64> for Tuple<K> {
    type Output = Tuple<K>;

    fn mul(self, _rhs: f64) -> Tuple<K> {
        &self * _rhs
    }
}


impl<K> std::ops::Div<f64> for &Tuple<K> {
    type Output = Tuple<K>;

    fn div(self, _rhs: f64) -> Tuple<K> {
        Tuple {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            kind: std::marker::PhantomData::<K>,
        }
    }
}


impl<K> std::ops::Div<f64> for Tuple<K> {
    type Output = Tuple<K>;

    fn div(self, _rhs: f64) -> Tuple<K> {
        &self / _rhs
    }
}


impl Tuple<Vector> {
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple<Vector> {
        Tuple { x, y, z, kind: std::marker::PhantomData::<Vector> }
    }

    pub fn random_in_unit_sphere() -> Tuple<Vector> {
        loop {
            let x = rand::random::<f64>() * 2.0 - 1.0;
            let y = rand::random::<f64>() * 2.0 - 1.0;
            let z = rand::random::<f64>() * 2.0 - 1.0;
            let vec = Tuple::vector(x, y, z);
            if vec.magnitude() < 1. {
                return vec
            }
        }
    }

    /// Returns a random vector in the unit disc of the x-y plane.
    pub fn random_in_unit_disc() -> Tuple<Vector> {
        loop {
            let x = rand::random::<f64>() * 2.0 - 1.0;
            let y = rand::random::<f64>() * 2.0 - 1.0;
            let vec = Tuple::vector(x, y, 0.0);
            if vec.magnitude() < 1. {
                return vec
            }
        }
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn unit(&self) -> Tuple<Vector> {
        self / self.magnitude()
    }

    pub fn cross(&self, _rhs: &Tuple<Vector>) -> Tuple<Vector> {
        Tuple::vector(
            self.y * _rhs.z - self.z * _rhs.y,
            self.z * _rhs.x - self.x * _rhs.z,
            self.x * _rhs.y - self.y * _rhs.x,
        )
    }

    pub fn dot(&self, _rhs: &Tuple<Vector>) -> f64 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }

}


impl Tuple<Point> {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple<Point> {
        Tuple { x, y, z, kind: std::marker::PhantomData::<Point> }
    }
}


impl<K: TupleType<K>> Tuple<K> {
    pub fn new(x: f64, y: f64, z: f64) -> Tuple<K> {
        K::make_tuple(x, y, z)
    }

    pub fn as_array(&self) -> [f64; 4] {
        K::as_array(self)
    }
}


#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use crate::eq;
    use super::*;
    use super::proptest_strategies::point as point_strat;
    use super::proptest_strategies::vector as vector_strat;


    proptest! {

        #[test]
        fn adding_vectors(
            a in vector_strat(100.),
            b in vector_strat(100.),
        ) {
            let c = Tuple::vector(a.x + b.x, a.y + b.y, a.z + b.z);
            assert_eq!(a + b, c);
        }

        #[test]
        fn subtracting_vector_from_point_returns_a_point(
            a in point_strat(100.),
            b in vector_strat(100.),
        ) {
            let c = Tuple::point(a.x - b.x, a.y - b.y, a.z - b.z);
            assert_eq!(a - b, c);
        }

        #[test]
        fn subtracting_vector_from_vector_returns_vector(
            a in vector_strat(100.),
            b in vector_strat(100.),
        ) {
            let c = Tuple::vector(a.x - b.x, a.y - b.y, a.z - b.z);
            assert_eq!(a - b, c);
        }

        #[test]
        fn negating_vector(
            x in any::<f64>(),
            y in any::<f64>(),
            z in any::<f64>(),
        ) {
            let a = Tuple::vector(x, y, z);
            let b = Tuple::vector(-x, -y, -z);
            assert_eq!(-a, b);
        }

        #[test]
        fn magnitude_of_unit_vector_is_one(
            a in vector_strat(100.)
        ) {
            assert!(eq(a.unit().magnitude(), 1.))
        }

        #[test]
        fn cross_product_is_perpendicular_to_inputs(
            a in vector_strat(100.),
            b in vector_strat(100.),
        ) {
            let c = a.cross(&b);
            assert!(eq(a.dot(&c), 0.));
            assert!(eq(b.dot(&c), 0.));
        }

    }


    #[test]
    fn multiplying_vector_by_scalar() {
        let a = Tuple::vector(1., -2., 3.);
        let b = Tuple::vector(3.5, -7., 10.5);
        assert_eq!(a * 3.5, b);
    }

    #[test]
    fn dividing_vector_by_scalar() {
        let a = Tuple::vector(1., -2., 3.);
        let b = Tuple::vector(0.5, -1., 1.5);
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

