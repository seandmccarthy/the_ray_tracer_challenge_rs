#[derive(PartialEq, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: i32,
}

pub fn run() {
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1 };
        assert_eq!(4.3, a.x);
        assert_eq!(-4.2, a.y);
        assert_eq!(3.1, a.z);
        assert_eq!(1, a.w);
        assert_eq!(point(4.3, -4.2, 3.1), a)
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0 };
        assert_eq!(4.3, a.x);
        assert_eq!(-4.2, a.y);
        assert_eq!(3.1, a.z);
        assert_eq!(0, a.w);
        assert_eq!(vector(4.3, -4.2, 3.1), a)
    }
}
