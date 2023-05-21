pub mod constants;
pub mod colors;
pub mod tuples;
pub mod canvas;
pub mod matrix;
pub mod transformations;
pub mod ray;
pub mod intersection;
pub mod object;
pub mod normal;
pub mod groups;
pub mod camera;
pub mod materials;
pub mod cli;
pub mod shapes;
pub mod raytrace;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < constants::EPSILON
}

pub fn minimum_by_key<'a, F, T, I>(mut iter: I, key: F) -> Option<&'a T>
    where F: Fn(&T) -> f64,
          I: std::iter::Iterator<Item = &'a T>
{
    let mut smallest = iter.next();
    if smallest.is_none() {
        return None;
    }
    loop {
        match iter.next() {
            None => return smallest,
            Some(x) => {
                if key(x) < key(smallest.unwrap()) {
                    smallest = Some(x);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_finds_minimum() {
        let xs: Vec<f64> = vec![1., 2., 3., 4., 5.];
        let val = minimum_by_key(xs.iter(), |x| *x);
        assert_eq!(val, Some(&1.));
    }

    #[test]
    fn minimum_returns_none_on_empty_iterator() {
        let xs: Vec<f64> = vec![];
        assert_eq!(minimum_by_key(xs.iter(), |x| *x), None);
    }
}
