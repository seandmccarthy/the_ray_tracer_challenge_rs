use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn colour(red: f64, green: f64, blue: f64) -> Colour {
    Colour { red, green, blue }
}

impl Add<Colour> for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Colour {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue
        }
    }
}

impl Sub for Colour {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Colour {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Colour {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other
        }
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Colour {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue
        }
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Colour) -> bool {
        let eps = 1e-6;
        (self.red - other.red).abs() < eps
            && (self.green - other.green).abs() < eps
            && (self.blue - other.blue).abs() < eps
            //&& (self.w - other.w).abs() < eps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_colour() {
        let c = colour(0.1, 0.2, 0.3);
        assert_eq!(0.1, c.red);
        assert_eq!(0.2, c.green);
        assert_eq!(0.3, c.blue);
    }

    #[test]
    fn adding_colours() {
        let c1 = colour(0.9, 0.6, 0.75);
        let c2 = colour(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, colour(1.6, 0.7, 1.0));
    }

    #[test]
    fn multiplying_a_colour_by_a_scalar() {
        let c = colour(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, colour(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colours() {
        let c1 = colour(1.0, 0.2, 0.4);
        let c2 = colour(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, colour(0.9, 0.2, 0.04));
    }
}
