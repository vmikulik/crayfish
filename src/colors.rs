
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


