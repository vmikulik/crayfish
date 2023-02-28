pub mod constants;
pub mod colors;
pub mod tuples;
pub mod canvas;
pub mod matrix;
pub mod transformations;
pub mod ray;
pub mod shapes;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < constants::EPSILON
}