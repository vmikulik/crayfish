
use crate::constants::EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    // These are between 0. and 1.0
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {red, green, blue}
    }

    pub fn from_u8(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red: red as f64 / u8::MAX as f64,
            green: green as f64 / u8::MAX as f64,
            blue: blue as f64 / u8::MAX as f64,
        }
    }
}


#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn from_u8_works_as_expected() {
        let c = Color::from_u8(0, 0, 255);
        assert_eq!(c.red, 0.);
        assert_eq!(c.green, 0.);
        assert_eq!(c.blue, 1.);
    }
}


impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color {
            red: self.red + _rhs.red,
            green: self.green + _rhs.green,
            blue: self.blue + _rhs.blue,
        }
    }
}


impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, _rhs: Color) -> Color {
        Color {
            red: self.red - _rhs.red,
            green: self.green - _rhs.green,
            blue: self.blue - _rhs.blue,
        }
    }
}


impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {
            red: self.red * _rhs.red,
            green: self.green * _rhs.green,
            blue: self.blue * _rhs.blue,
        }
    }
}


impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color {
            red: self.red * _rhs,
            green: self.green * _rhs,
            blue: self.blue * _rhs,
        }
    }
}


impl std::cmp::PartialEq for Color {
    fn eq(&self, _rhs: &Color) -> bool {
        (self.red - _rhs.red).abs() < EPSILON
        && (self.green - _rhs.green).abs() < EPSILON
        && (self.blue - _rhs.blue).abs() < EPSILON
    }
}


