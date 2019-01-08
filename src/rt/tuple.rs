use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Add<Tuple> for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

pub fn run() {
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub fn colour(red: f64, green: f64, blue: f64) -> Tuple {
    Tuple { x: red, y: green, z: blue, w: 0.0 }
}

pub fn magnitude(v: &Tuple) -> f64 {
    (v.x.powi(2) + v.y.powi(2) + v.z.powi(2) + v.w.powi(2)).sqrt()
}

pub fn normalise(v: &Tuple) -> Tuple {
    let mag = magnitude(v);
    Tuple {
        x: v.x / mag,
        y: v.y / mag,
        z: v.z / mag,
        w: v.w / mag
    }
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    vector(a.y * b.z - a.z * b.y,
           a.z * b.x - a.x * b.z,
           a.x * b.y - a.y * b.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
        assert_eq!(4.3, a.x);
        assert_eq!(-4.2, a.y);
        assert_eq!(3.1, a.z);
        assert_eq!(1.0, a.w);
        assert_eq!(point(4.3, -4.2, 3.1), a)
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
        assert_eq!(4.3, a.x);
        assert_eq!(-4.2, a.y);
        assert_eq!(3.1, a.z);
        assert_eq!(0.0, a.w);
        assert_eq!(vector(4.3, -4.2, 3.1), a)
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple { x: 3.0, y: -2.0, z: 5.0, w: 1.0 };
        let a2 = Tuple { x: -2.0, y: 3.0, z: 1.0, w: 0.0 };
        assert_eq!(Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 }, a1 + a2);
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(vector(-2.0, -4.0, -6.0), v1 - v2);
    }

    #[test]
    fn negating_a_tuple() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple(-1.0, 2.0, -3.0, 4.0), -a);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple(3.5, -7.0, 10.5, -14.0), a * 3.5)
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple(0.5, -1.0, 1.5, -2.0), a * 0.5)
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(tuple(0.5, -1.0, 1.5, -2.0), a / 2.0)
    }

    #[test]
    fn magnitude_of_vector_x() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(magnitude(&v), 1.0)
    }

    #[test]
    fn magnitude_of_vector_y() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(magnitude(&v), 1.0)
    }

    #[test]
    fn magnitude_of_vector_z() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(magnitude(&v), 1.0)
    }

    #[test]
    fn magnitude_of_vector() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(magnitude(&v), 14.0_f64.sqrt())
    }

    #[test]
    fn magnitude_of_negative_vector() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(magnitude(&v), 14.0_f64.sqrt())
    }

    #[test]
    fn normalising_a_vector_x_only() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(normalise(&v), vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn normalising_a_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let mag = magnitude(&v);
        assert_eq!(normalise(&v), vector(1.0 / mag, 2.0 / mag, 3.0 / mag))
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(dot(&v1, &v2), 20.0_f64);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(cross(&v1, &v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(cross(&v2, &v1), vector(1.0, -2.0, 1.0));
    }
}

