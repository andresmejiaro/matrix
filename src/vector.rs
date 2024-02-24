use crate::matrix::Matrix;
use crate::traits::{Conj, Norm, One, Tf64};
use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + PartialEq
        + Copy
        + Default
        + One
        + Tf64
        + Norm
        + Neg
        + Conj
        + std::fmt::Display,
{
    size: usize,
    matrix: Matrix<K>,
}

impl<K> Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + PartialEq
        + Copy
        + Default
        + Tf64
        + Norm
        + One
        + Neg
        + Conj
        + std::fmt::Display,
{
    pub fn new(elements: Vec<K>) -> Vector<K> {
        assert!(elements.len() > 0, "Input is empty");
        let n = elements.len();

        Vector {
            size: n,
            matrix: Matrix::new(elements, n, 1),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn matrix(&self) -> &Matrix<K> {
        &self.matrix
    }

    pub fn el(&self, i: usize) -> K {
        let n = self.size;
        assert!(i <= n);
        self.matrix.el(i, 1)
    }

    pub fn zero(n: usize) -> Vector<K> {
        Vector::<K>::new(vec![K::default(); n])
    }

    pub fn add(&self, other: &Vector<K>) -> Vector<K> {
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let n = self.size;
        let to_return =
            Vector::<K>::new(self.matrix.add(&other.matrix).elements);
        to_return
    }

    pub fn scl(&self, scaling: K) -> Vector<K> {
        let n = self.size;
        Vector::new(self.matrix.scl(scaling).elements)
    }

    pub fn sub(&self, other: &Vector<K>) -> Vector<K> {
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let n = self.size;
        Vector::<K>::new(self.matrix.sub(&other.matrix).elements)
    }

    pub fn linear_combination(u: &[&Vector<K>], coefs: &[K]) -> Vector<K> {
        assert!(
            u.len() != 0 && coefs.len() != 0,
            "Empty data for lineal combination"
        );
        assert_eq!(
            u.len(),
            coefs.len(),
            "Number of vectors and coefficients doesn't match for linear combination"
        );
        let mut to_return = Vector::<K>::zero(u[0].size);
        for (v, s) in u.iter().zip(coefs.iter()) {
            to_return = to_return.add(&v.scl(*s));
        }
        to_return
    }

    pub fn lerp(u: &Vector<K>, v: &Vector<K>, t: K) -> Vector<K> {
        Vector::<K>::linear_combination(&[u, v], &[K::one() - t, t])
    }

    pub fn dot(&self, other: &Vector<K>) -> K {
        assert_eq![self.size(), other.size(), "dot product: sizes don't match"];
        if self.size() == 0 {
            return K::default();
        } else {
            let m1 = self.matrix.adj().mlt(&other.matrix);
            m1.el(1, 1)
        }
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).tf64().powf(0.5)
    }

    pub fn norm_1(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.matrix.elements {
            x += i.norm();
        }
        x
    }

    pub fn norm_inf(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.matrix.elements {
            let x2 = i.norm().tf64();
            if x2 > x {
                x = x2;
            }
        }
        x
    }

    pub fn angle_cos(&self, other: &Self) -> f64 {
        self.dot(other).tf64() / (self.norm() * other.norm())
    }

    pub fn cross_product(&self, other: &Self) -> Vector<K> {
        assert_eq!(self.size(), 3, "Wrong size for cross product");
        assert_eq!(self.size(), other.size(), "Wrong size for cross product");
        let c1 = self.el(2) * other.el(3) - other.el(2) * self.el(3);
        let c2 = (K::default() - self.el(1)) * other.el(3)
            + self.el(3) * other.el(1);
        let c3 = self.el(1) * other.el(2) - self.el(2) * other.el(1);
        Vector::new(vec![c1, c2, c3])
    }
}

impl<K> fmt::Display for Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Div<Output = K>
        + PartialEq
        + Copy
        + Default
        + fmt::Display
        + One
        + Tf64
        + Norm
        + Neg
        + Conj,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.size();
        write!(f, "[")?;
        for j in 1..=n {
            write!(f, "{}", self.matrix.el(j, 1))?;
            if j != n {
                write![f, ","]?;
            }
        }
        write!(f, "]")
    }
}
