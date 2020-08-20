use crate::tuple::Tuple;
use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple<f32>,
    pub dir: Tuple<f32>,
}

impl Ray {
    pub fn new(origin: Tuple<f32>, dir: Tuple<f32>) -> Self {
        Self { origin, dir: dir.normalize() }
    }

    pub fn pos(&self, t: f32) -> Tuple<f32> {
        self.origin + self.dir * t
    }

    pub fn transform(&self, m: &Matrix<f32>) -> Self {
        // don't call Ray::new because it normalizes dir
        // Found the bug!
        Self { origin: m * self.origin, dir: m * self.dir }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let r = Ray::new(Tuple::new_point(1.0, 2.0, 3.0), Tuple::new_vector(0.0, 1.0, 0.0));
        let m = Matrix::translate(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);

        assert!(r2.origin == Tuple::new_point(4.0, 6.0, 8.0));
        assert!(r2.dir == Tuple::new_vector(0.0, 1.0, 0.0));

        let r = Ray::new(Tuple::new_point(0.0, 0.0, -5.0), Tuple::new_vector(0.0, 0.0, 1.0));
        let m = Matrix::scale(2.0, 2.0, 2.0);
        let r2 = r.transform(&m);
        assert!(r2.origin == Tuple::new_point(0.0, 0.0, -10.0));
        assert!(r2.dir == Tuple::new_vector(0.0, 0.0, 2.0));
    }
}
