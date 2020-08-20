use std::ops;
use num_traits::Num;
use num_traits::identities::{One, Zero};
use num_traits::real::Real;

#[derive(Clone, Copy, Debug, Eq, PartialEq)] // just four floating numbers
pub struct Tuple<T>(pub T, pub T, pub T, pub T);

impl<T: Num + Copy> Tuple<T> {
    pub fn new_vec(v: Vec<T>) -> Self {
        assert!(v.len() == 4);
        Self(v[0], v[1], v[2], v[3])
    }
}


impl<T: Num + One> Tuple<T> {
    pub fn new_point(x: T, y: T, z: T) -> Self {
        Self(x, y, z, T::one())
    }
}

impl<T: Num + Zero> Tuple<T> {
    pub fn new_vector(x: T, y: T, z: T) -> Self {
        Self(x, y, z, T::zero())
    }
}

impl<T: Num + Zero + Real> Tuple<T> {
    pub fn magnitude(self: Tuple<T>) -> T {
        assert!(self.3 == T::zero(), "This is not a vector");
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(self: Tuple<T>) -> Tuple<T> {
        assert!(self.3 == T::zero(), "This is not a vector");
        self / self.magnitude()
    }
}


impl<T: Num + One + Copy> Tuple<T> {
    pub fn reflect(&self, n: &Tuple<T>) -> Tuple<T> {
        *self - *n * (T::one() + T::one()) * dot(*self, *n)
    }
}

pub fn dot<T: Num + Zero>(t1: Tuple<T>, t2: Tuple<T>) -> T {
    assert!(t1.3 == T::zero(), "Vector t1 is not a vector");
    assert!(t2.3 == T::zero(), "Vector t2 is not a vector");
    t1.0 * t2.0 + t1.1 * t2.1 + t1.2 * t2.2 + t1.3 * t2.3
}

pub fn cross<T: Num + Zero + Copy>(t1: Tuple<T>, t2: Tuple<T>) -> Tuple<T> {
    assert!(t1.3 == T::zero(), "Vector t1 is not a vector");
    assert!(t2.3 == T::zero(), "Vector t2 is not a vector");
    Tuple::new_vector(
        t1.1 * t2.2 - t1.2 * t2.1,
        t1.2 * t2.0 - t1.0 * t2.2,
        t1.0 * t2.1 - t1.1 * t2.0,
    )
}

impl<T: Num + Real> Tuple<T> {
    pub fn eq_real(&self, other: &Tuple<T>) -> bool {
        return 
            (self.0 - other.0).abs() <= T::epsilon() && 
            (self.1 - other.1).abs() <= T::epsilon() && 
            (self.2 - other.2).abs() <= T::epsilon() && 
            (self.3 - other.3).abs() <= T::epsilon();
    }
}

impl<T: Num> ops::Add<Tuple<T>> for Tuple<T> {
    type Output = Tuple<T>;

    fn add(self, other: Tuple<T>) -> Tuple<T> {
        Tuple(
            self.0 + other.0, 
            self.1 + other.1, 
            self.2 + other.2, 
            self.3 + other.3
            )
    }
}

impl<T: Num> ops::Sub<Tuple<T>> for Tuple<T> {
    type Output = Tuple<T>;

    fn sub(self, other: Tuple<T>) -> Tuple<T> {
         Tuple(
            self.0 - other.0, 
            self.1 - other.1, 
            self.2 - other.2, 
            self.3 - other.3
            )
    }
}

impl<T: Num + Zero> ops::Neg for Tuple<T> {
    type Output = Tuple<T>;

    fn neg(self) -> Tuple<T> {
        Tuple(
            T::zero() - self.0, 
            T::zero() - self.1, 
            T::zero() - self.2, 
            T::zero() - self.3
            )
    }
}

impl<T: Num + Copy> ops::Mul<T> for Tuple<T> {
    type Output = Tuple<T>;

    fn mul(self, other: T) -> Tuple<T> {
        Tuple(
            self.0 * other, 
            self.1 * other, 
            self.2 * other, 
            self.3 * other
            )
    }
}

impl<T: Real> ops::Div<T> for Tuple<T> {
    type Output = Tuple<T>;

    fn div(self, other: T) -> Tuple<T> {
        Tuple(
            self.0 / other, 
            self.1 / other, 
            self.2 / other, 
            self.3 / other
            )
    }
}

impl<T> ops::Index<usize> for Tuple<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < 4);
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> ops::IndexMut<usize> for Tuple<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx < 4);
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
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

    #[test]
    fn test_reflect() {
        let a = Tuple::new_vector(0.0, -1.0, 0.0);
        let n = Tuple::new_vector(2.0.sqrt() / 2.0, 2.0.sqrt() / 2.0, 0.0);
        assert!(a.reflect(&n).eq_real(&Tuple::new_vector(1.0, 0.0, 0.0)));
    }
}
