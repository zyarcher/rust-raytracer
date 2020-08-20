use std::ops;
use crate::tuple::Tuple;
use crate::tuple::cross;
use num_traits::Num;
use num_traits::{Zero, One};
use num_traits::real::Real;

// stored in column major mode
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T> {
    elems: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Matrix<T> {
    pub fn new(elems: Vec<Vec<T>>) -> Self {
        assert!(elems.len() > 0);
        Self {
            elems: elems.concat(),
            width: elems.len(),
            height: elems[0].len(),
        }
    }
}

impl<T: Real> Matrix<T> {
    pub fn eq_real(&self, other: Matrix<T>) -> bool {
        self.elems.iter().zip(other.elems.iter())
            .all(|(&a, &b)| {
                (a - b).abs() <= T::epsilon()
            })
    }
}
    
impl<T: Zero> Matrix<T> {
    pub fn zeros(width: usize, height: usize) -> Self {
        Self::new_fn(|_, _| T::zero(), width, height)
    }
}

impl<T: One + Zero> Matrix<T> {
    pub fn eye(size: usize) -> Self {
        Self::new_fn(|i, j| if i == j { T::one() } else { T::zero() }, size, size)
    }
}

impl<T> Matrix<T> {
    pub fn new_fn_option<F>(f: F, width: usize, height: usize) -> Option<Self>
    where F: Fn(usize, usize) -> Option<T> {
        let mut elems: Vec<T> = Vec::with_capacity(width * height);
        for j in 0..height {
            for i in 0..width {
                let a = f(j, i);
                match a {
                    None => return None,
                    Some(a) => elems.push(a),
                }
            }
        }

        Some(Self { elems, width, height })
    }
    pub fn new_fn<F>(mut f: F, width: usize, height: usize) -> Self 
    where F: FnMut(usize, usize) -> T {
        let mut elems: Vec<T> = Vec::with_capacity(width * height);
        for j in 0..height {
            for i in 0..width {
                elems.push(f(j, i));
            }
        }

        Self { elems, width, height }
    }

    fn get_index(&self, i: usize, j: usize) -> usize {
        assert!(i < self.height && j < self.width);
        i * self.width + j
    }
}

impl<T: Copy> Matrix<T> {
    pub fn transpose(&self) -> Matrix<T> {
        Matrix::new_fn(|i, j| self[(j, i)], self.width, self.height)
    }
}

impl<T: Num + Zero> Matrix<T> {
    pub fn translate(x: T, y: T, z: T) -> Self {
        let mut eye = Self::eye(4);
        eye[(0, 3)] = x;
        eye[(1, 3)] = y;
        eye[(2, 3)] = z;
        return eye
    }

    pub fn scale(x: T, y: T, z: T) -> Self {
        let mut eye = Self::eye(4);
        eye[(0, 0)] = x;
        eye[(1, 1)] = y;
        eye[(2, 2)] = z;
        return eye
    }
}

impl<T: Real + Zero> Matrix<T> {
    pub fn rotation_x(r: T) -> Self {
        let mut eye = Self::eye(4);
        eye[(1,1)] = r.cos();
        eye[(1,2)] = -r.sin();
        eye[(2,1)] = r.sin();
        eye[(2,2)] = r.cos();
        return eye
    }

    pub fn rotation_y(r: T) -> Self {
        let mut eye = Self::eye(4);
        eye[(0, 0)] = r.cos();
        eye[(0, 2)] = r.sin();
        eye[(2, 0)] = -r.sin();
        eye[(2, 2)] = r.cos();
        return eye
    }

    pub fn rotation_z(r: T) -> Self {
        let mut eye = Self::eye(4);
        eye[(0, 0)] = r.cos();
        eye[(0, 1)] = -r.sin();
        eye[(1, 0)] = r.sin();
        eye[(1, 1)] = r.cos();
        return eye
    }
}

impl<T: Real + Zero + std::iter::Sum> Matrix<T> {
    pub fn view_transform(from: Tuple<T>, to: Tuple<T>, up: Tuple<T>) -> Self {
        let up = up.normalize();
        let forward = (to - from).normalize();
        let left = cross(forward, up);
        let real_up = cross(left, forward);

        &Self::new(
            vec![
                vec![left.0,    left.1,       left.2,      T::zero()],
                vec![real_up.0, real_up.1,    real_up.2,   T::zero()],
                vec![-forward.0, -forward.1,  -forward.2,  T::zero()],
                vec![T::zero(), T::zero(),    T::zero(),   T::one()],
            ]
        ) * &Self::translate(-from.0, -from.1, -from.2)
    }
}

impl<T: Num + Copy + Zero + std::iter::Sum> Matrix<T> {
    pub fn det(&self) -> T {
        if self.width == 1 && self.height == 1 {
            self[(0, 0)]
        } else if self.width == 2 && self.height == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            (0..self.width)
                .map(|col| {
                    self.cofactor(0, col) * self[(0, col)]
                })
                .sum()
        }
    }

    // return new matrix with row and col removed
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<T> {
        let mut new_mat = Self::new_fn(|_, _| T::zero(), self.width - 1, self.height - 1);
        for r in 0..row {
            for c in 0..col {
                new_mat[(r, c)] = self[(r, c)];
            }

            for c in (col + 1)..self.width {
                new_mat[(r, c - 1)] = self[(r, c)];
            }
        }

        for r in (row + 1)..self.height {
            for c in 0..col {
                new_mat[(r - 1, c)] = self[(r, c)];
            }

            for c in (col + 1)..self.width {
                new_mat[(r - 1, c - 1)] = self[(r, c)];
            }
        }

        new_mat
    }

    pub fn mirror(&self, row: usize, col: usize) -> T {
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let sign = if (row + col) % 2 == 0 { T::one() } else { T::zero() - T::one() };
        self.submatrix(row, col).det() * sign
    }
}

impl<T: Num + Copy + Zero + std::iter::Sum + Real> Matrix<T> {
    pub fn inverse(&self) -> Option<Matrix<T>> {
        let d = self.det();
        Matrix::new_fn_option(|i, j| {
            if d.abs() < T::epsilon() {
                None
            } else {
                Some(self.cofactor(j, i) / d) // transpose matrix, flip j and i
            }
        }, self.width, self.height)
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.elems[self.get_index(idx.0, idx.1)]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let i = self.get_index(idx.0, idx.1);
        &mut self.elems[i]
    }
}

impl<T: Num + std::iter::Sum + Copy> ops::Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: Self) -> Self::Output {
        assert!(self.height == other.width);
        Matrix::new_fn(|i, j| {
            (0..self.height)
                .map(|k| {
                    self[(i, k)] * other[(k, j)]
                })
                .sum()
        }, self.width, other.height)
    }
}

impl<T: Num + std::iter::Sum + Copy> ops::Mul<Tuple<T>> for &Matrix<T> {
    type Output = Tuple<T>;

    fn mul(self, other: Tuple<T>) -> Self::Output {
        assert!(self.height == 4);
        Tuple::new_vec(
            (0..self.height)
                .map(|i| {
                    (0..self.width).map(|j| self[(i, j)] * other[j]).sum()
                }).collect()
        )
    }
}

impl<T: quickcheck::Arbitrary> quickcheck::Arbitrary for Matrix<T> {
    fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
        let w = usize::arbitrary(g);
        let h = usize::arbitrary(g);
        Matrix::new_fn(|_, _| T::arbitrary(g), w, h)
    }
}

pub struct TransformBuilder<T>(Matrix<T>);
impl<T: Real + One + Zero + Num + std::iter::Sum> TransformBuilder<T> {
    pub fn identity() -> Self {
        TransformBuilder(Matrix::eye(4))
    }

    pub fn translate(&self, x: T, y: T, z: T) -> Self {
        TransformBuilder(&Matrix::translate(x, y, z) * &self.0)
    }

    pub fn scale(&self, x: T, y: T, z: T) -> Self {
        TransformBuilder(&Matrix::scale(x, y, z) * &self.0)
    }

    pub fn rotation_x(&self, r: T) -> Self {
        TransformBuilder(&Matrix::rotation_x(r) * &self.0)
    }

    pub fn rotation_y(&self, r: T) -> Self {
        TransformBuilder(&Matrix::rotation_y(r) * &self.0)
    }

    pub fn rotation_z(&self, r: T) -> Self {
        TransformBuilder(&Matrix::rotation_z(r) * &self.0)
    }

    pub fn build(self) -> Matrix<T> {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_mat() {
        let m = Matrix::new(
            vec![
                vec![-3, 5],
                vec![1, -2],
            ]
        );
        assert!(m[(0, 0)] == -3);
        assert!(m[(0, 1)] == 5);
        assert!(m[(1, 0)] == 1);
        assert!(m[(1, 1)] == -2);

        let m = Matrix::new(
            vec![
                vec![-3, 5, 0],
                vec![1, -2, -7],
                vec![0, 1, 1],
            ]
        );
        assert!(m[(0, 0)] == -3);
        assert!(m[(1, 1)] == -2);
        assert!(m[(2, 2)] == 1);
    }

    #[test]
    fn test_mat_eq() {
        let m1 = Matrix::new(
            vec![
                vec![-3, 5, 0],
                vec![1, -2, -7],
                vec![0, 1, 1],
            ]
        );
        let m2 = Matrix::new(
            vec![
                vec![-3, 5, 0],
                vec![1, -2, -7],
                vec![0, 1, 1],
            ]
        );
        assert!(m1 == m2);
        let m2 = Matrix::new(
            vec![
                vec![-3, 5, 0],
                vec![1, -2, -7],
                vec![0, 1, 2],
            ]
        );
        assert!(m1 != m2);
    }

    #[test]
    fn test_mat_mul() {
        let m1 = Matrix::new(
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 8, 7, 6],
                vec![5, 4, 3, 2],
            ]
        );

        let m2 = Matrix::new(
            vec![
                vec![-2, 1, 2, 3],
                vec![ 3, 2, 1,-1],
                vec![ 4, 3, 6, 5],
                vec![ 1, 2, 7, 8],
            ]
        );

        let m3 = Matrix::new(
            vec![
                vec![20,  22,  50, 48],
                vec![44,  54, 114,108],
                vec![40,  58, 110,102],
                vec![16,  26,  46, 42],
            ]
        );

        assert!(&m1 * &m2 == m3);

        let m = Matrix::new(
            vec![
                vec![1, 2, 3,4],
                vec![2, 4, 4,2],
                vec![8, 6, 4,1],
                vec![0, 0, 0,1],
            ]
        );
        let b = Tuple(1, 2, 3, 1);
        println!("{:?}", &m * b);
        assert!(&m * b == Tuple(18, 24, 33, 1));
    }

    #[test]
    fn test_sub() {
        let m = Matrix::new(
            vec![
                vec![1, 5, 0],
                vec![-3, 2, 7],
                vec![0, 6, -3],
            ]
        );

        let submatrix = Matrix::new(
            vec![
                vec![-3, 2],
                vec![0, 6],
            ]
        );
        assert!(m.submatrix(0, 2) == submatrix);

        let m = Matrix::new(
            vec![
                vec![-6,  1,  1, 6],
                vec![-8,  5,  8, 6],
                vec![-1,  0,  8, 2],
                vec![-7,  1, -1, 1],
            ]
        );

        let submatrix = Matrix::new(
            vec![
                vec![-6,  1, 6],
                vec![-8,  8, 6],
                vec![-7, -1, 1],
            ]
        );
        assert!(m.submatrix(2, 1) == submatrix);
    }

    #[test]
    fn test_cofactor_mirror() {
        let m = Matrix::new(
            vec![
                vec![-3, 5, 0],
                vec![2, -1, -7],
                vec![6, -1, 5],
            ]
        );
        assert!(m.mirror(0, 0) == -12);
        assert!(m.cofactor(0, 0) == -12);
        assert!(m.mirror(1, 0) == 25);
        assert!(m.cofactor(1, 0) == -25);
    }

    #[test]
    fn test_det() {
        let m = Matrix::new(
            vec![
                vec![1, 5],
                vec![-3, 2],
            ]
        );
        assert!(m.det() == 17);
    }

    #[test]
    fn test_inverse() {
        let m = Matrix::new(
            vec![
                vec![1.0, 5.0],
                vec![-3.0, 2.0],
            ]
        );
        println!("{:?}", m.inverse().unwrap());
        println!("{:?}", m.inverse().unwrap().inverse().unwrap());
        assert!(m.eq_real(m.inverse().unwrap().inverse().unwrap()));

        let m = Matrix::scale(2.0, 2.0, 2.0);
        let m_inv = Matrix::scale(0.5, 0.5, 0.5);
        assert!(m.inverse().unwrap().eq_real(m_inv));
    }

    #[test]
    fn test_rot() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f32::consts::PI / 4.0);
        let inv_half_quarter = half_quarter.inverse().unwrap();
        assert!(((&inv_half_quarter) * p).eq_real(&Tuple::new_point(0.0, 2.0.sqrt() / 2.0, -2.0.sqrt()/2.0)));

        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(std::f32::consts::PI / 4.0);
        assert!(((&half_quarter) * p).eq_real(&Tuple::new_point(2.0.sqrt() / 2.0, 0.0, 2.0.sqrt()/2.0)));

        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(std::f32::consts::PI / 4.0);
        assert!(((&half_quarter) * p).eq_real(&Tuple::new_point(-2.0.sqrt() / 2.0, 2.0.sqrt()/2.0, 0.0)));
    }

    #[test]
    fn test_view() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, -1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert!(t == Matrix::eye(4));

        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, 1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert!(t == Matrix::scale(-1.0, 1.0, -1.0));
    }
}
