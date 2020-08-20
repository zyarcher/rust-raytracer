use std::ops;

#[derive(Clone, Copy, Debug)] // just four floating numbers
pub struct Tuple(pub f32, pub f32, pub f32, pub f32);

impl Tuple {
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 0.0)
    }

    pub fn magnitude(self: Tuple) -> f32 {
        assert!(self.3 == 0.0, "This is not a vector");
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(self: Tuple) -> Tuple {
        assert!(self.3 == 0.0, "This is not a vector");
        self / self.magnitude()
    }
}

pub fn dot(t1: Tuple, t2: Tuple) -> f32 {
    assert!(t1.3 == 0.0, "Vector t1 is not a vector");
    assert!(t2.3 == 0.0, "Vector t2 is not a vector");
    t1.0 * t2.0 + t1.1 * t2.1 + t1.2 * t2.2 + t1.3 * t2.3
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        return 
            (self.0 - other.0).abs() < f32::EPSILON && 
            (self.1 - other.1).abs() < f32::EPSILON && 
            (self.2 - other.2).abs() < f32::EPSILON && 
            (self.3 - other.3).abs() < f32::EPSILON;
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple(
            self.0 + other.0, 
            self.1 + other.1, 
            self.2 + other.2, 
            self.3 + other.3
            )
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
         Tuple(
            self.0 - other.0, 
            self.1 - other.1, 
            self.2 - other.2, 
            self.3 - other.3
            )
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple(
            -self.0, 
            -self.1, 
            -self.2, 
            -self.3
            )
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f32) -> Tuple {
        Tuple(
            self.0 * other, 
            self.1 * other, 
            self.2 * other, 
            self.3 * other
            )
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, other: f32) -> Tuple {
        Tuple(
            self.0 / other, 
            self.1 / other, 
            self.2 / other, 
            self.3 / other
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_vec() {
        assert!(Tuple::new_point(4.0, -4.0, 3.0) == Tuple(4.0, -4.0, 3.0, 1.0));
        assert!(Tuple::new_vector(4.0, -4.0, 3.0) == Tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn test_add() {
        let a1 = Tuple::new_point(3.0, -2.0, 5.0);
        let a2 = Tuple::new_vector(-2.0, 3.0, 1.0);
        assert!(a1 + a2 == Tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn test_mul_div() {
        let a = Tuple(1.0, -2.0, 3.0, -4.0);
        assert!(a / 2.0 == Tuple(0.5, -1.0, 1.5, -2.0));
        assert!(a * 2.0 == Tuple(2.0, -4.0, 6.0, -8.0));
    }

    #[test]
    fn test_mag() {
        let a = Tuple::new_vector(1.0, -2.0, -3.0);
        assert!(a.magnitude() == 14.0_f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        let mut a = Tuple::new_vector(4.0, 0.0, 0.0);
        assert!(a.normalize() == Tuple::new_vector(1.0, 0.0, 0.0));

        a = a.normalize();
        assert!(a.magnitude() == 1.0);
    }

    #[test]
    fn test_dot() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        assert!(dot(a, b) == 20.0);
    }
}
