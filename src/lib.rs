pub mod constants;
pub mod colors_lib;
pub mod tuples_lib;
pub mod canvas_lib;
pub mod matrix_lib;
pub mod transformations_lib;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn eq(a: f64, b: f64) -> bool {
    (a - b).abs() < constants::EPSILON
}