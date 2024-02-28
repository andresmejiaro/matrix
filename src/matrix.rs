use core::fmt;
use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    result,
};

use crate::{
    traits::{Conj, Norm, One, Tf64},
    vector::Vector,
};

#[derive(PartialEq, Clone)]
pub struct Matrix<K>
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
        + Conj
        + Neg,
{
    size: (usize, usize),
    pub elements: Vec<K>,
}

impl<K> Matrix<K>
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
        + Conj
        + Neg
        + std::fmt::Display,
{
    pub fn new(elements: Vec<K>, n: usize, m: usize) -> Matrix<K> {
        assert!(
            elements.len() == n * m,
            "The number of elements does not match!"
        );

        Matrix {
            size: (n, m),
            elements,
        }
    }

    

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn el(&self, i: usize, j: usize) -> K {
        let (n, m) = self.size;
        assert!(i <= n && j <= m);
        self.elements[i - 1 + (j - 1) * n]
    }

    pub fn set(&mut self, i: usize, j: usize, val: K) {
        let (n, m) = self.size;
        assert!(i <= n && j <= m);
        self.elements[i - 1 + (j - 1) * n] = val;
    }

    pub fn tr(&self) -> Matrix<K> {
        let (n, m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m * n], m, n);
        for i in 1..=m {
            for j in 1..=n {
                to_return.set(i, j, self.el(j, i));
            }
        }
        to_return
    }

    pub fn adj(&self) -> Matrix<K> {
        let (n, m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m * n], m, n);
        for i in 1..=m {
            for j in 1..=n {
                to_return.set(i, j, self.el(j, i).conj());
            }
        }
        to_return
    }

    pub fn zero(m: usize, n: usize) -> Matrix<K> {
        Matrix::<K>::new(vec![K::default(); m * n], m, n)
    }

    pub fn add(&self, other: &Matrix<K>) -> Matrix<K> {
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let (n, m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m * n], n, m);
        for i in 1..=n {
            for j in 1..=m {
                to_return.set(i, j, self.el(i, j) + other.el(i, j));
            }
        }
        to_return
    }

    pub fn scl(&self, scaling: K) -> Matrix<K> {
        let (n, m) = self.size;
        let mut to_return = Matrix::zero(n, m);
        for i in 1..=n {
            for j in 1..=m {
                to_return.set(i, j, scaling * self.el(i, j));
            }
        }
        to_return
    }

    pub fn lerp(u: &Matrix<K>, v: &Matrix<K>, t: K) -> Matrix<K> {
        Matrix::<K>::linear_combination(&[u, v], &[K::one() - t, t])
    }

    pub fn sub(&self, other: &Matrix<K>) -> Matrix<K> {
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let (n, m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m * n], n, m);
        for i in 1..=n {
            for j in 1..=m {
                to_return.set(i, j, self.el(i, j) - other.el(i, j));
            }
        }
        to_return
    }

    pub fn append_horizontal(&self, other: &Matrix<K>) -> Matrix<K> {
        let (n1, m1) = self.size;
        let (n2, m2) = other.size;
        assert_eq!(n1, n2, "Sizes not compatible for joining");
        Matrix::new(
            self.elements
                .iter()
                .cloned()
                .chain(other.elements.iter().cloned())
                .collect(),
            n1,
            m1 + m2,
        )
    }

    pub fn mlt(&self, other: &Matrix<K>) -> Matrix<K> {
        let (n, p1) = self.size();
        let (p2, m) = other.size();
        assert_eq!(p1, p2, "Size for multiplication not compatible");
        let mut newv: Vec<K> = (0..m * n).map(|_| K::default()).collect();
        for i in 0..n {
            for j in 0..m {
                for w in 0..p1 {
                    //println!("i {i} j {j} w {w} m {m} n {n}");
                    newv[j * n + i] = newv[j * n + i]
                        + self.el(i + 1, w + 1) * other.el(w + 1, j + 1);
                }
            }
        }
        Matrix::<K>::new(newv, n, m)
    }

    pub fn mul_vec(&self, other: &Vector<K>) -> Vector<K> {
        let result = self.mlt(other.matrix());
        Vector::<K>::new(result.elements)
    }

    pub fn mul_mat(&self, other: &Matrix<K>) -> Matrix<K> {
        self.mlt(other)
    }

    pub fn linear_combination(u: &[&Matrix<K>], coefs: &[K]) -> Matrix<K> {
        assert!(
            u.len() != 0 && coefs.len() != 0,
            "Empty data for lineal combination"
        );
        assert_eq!(
            u.len(),
            coefs.len(),
            "Number of vectors and coefficients doesn't match for linear combination"
        );
        let (m, n) = u[0].size();
        let mut to_return = Matrix::<K>::zero(m, n);
        for (v, s) in u.iter().zip(coefs.iter()) {
            to_return = to_return.add(&v.scl(*s));
        }
        to_return
    }

    pub fn trace(&self) -> K {
        let (m, n) = self.size();
        assert_eq!(m, n, "Trace must take a square matrix");
        let mut to_return = K::default();
        for i in 1..=n {
            to_return = to_return + self.el(i, i);
        }
        to_return
    }

    fn row_scaling(&mut self, row: usize, cnt: K) -> K {
        let (n, m) = self.size();
        assert!(row <= n && row != 0, "row not in range");
        for i in 1..=m {
            self.set(row, i, cnt * self.el(row, i));
        }
        K::one() / cnt
    }

    fn row_swapping(&mut self, row1: usize, row2: usize) -> K {
        let (n, m) = self.size();
        assert!(row1 <= n && row1 != 0, "row not in range");
        assert!(row2 <= n && row2 != 0, "row not in range");
        for i in 1..=m {
            let int = self.el(row1, i);
            self.set(row1, i, self.el(row2, i));
            self.set(row2, i, int);
        }
        K::default() - K::one()
    }

    fn row_static_add(&mut self, dest: usize, org: usize, factor: K) -> K {
        let (n, m) = self.size();
        assert!(dest <= n && dest != 0, "row not in range");
        assert!(org <= n && org != 0, "row not in range");
        for i in 1..=m {
            self.set(dest, i, self.el(dest, i) + factor * self.el(org, i));
        }
        K::one()
    }

    fn gauss_red_det_rank(&mut self, inv: bool) -> (K, usize, Matrix<K>) {
        let (n, m) = self.size();
        let mut det = K::one();
        let mut pivot_row = 0;
        let mut pivot_col = 0;

        let mut inv_m;

        if inv {
            inv_m = Matrix::<K>::identity(n);
        } else {
            inv_m = Matrix::<K>::identity(1);
        }

        // in each column
        'outer: for col in 1..=m {
            if pivot_row >= n {
                break;
            }
            //mare sure thar there is a pivot
            if self.el(pivot_row + 1, col) != K::default() {
                pivot_row += 1;
                pivot_col = col;
            } else {
                // look for a new pivot
                for row in (pivot_row + 1)..=n {
                    if self.el(row, col) != K::default() {
                        if inv {
                            _ = inv_m.row_swapping(row, pivot_row + 1);
                        }
                        det = det * self.row_swapping(row, pivot_row + 1);
                        pivot_row += 1;
                        pivot_col = col;
                        break;
                    }
                    if row == n {
                        // no pivot found
                        det = K::default();
                        if inv {
                            break;
                        } else {
                            continue 'outer;
                        }
                    }
                }
            }
            // Normalize the new pivot and reduce the column.
            let scaling = K::one() / self.el(pivot_row, pivot_col);
            if inv{
                _ = inv_m.row_scaling(
                    pivot_row,
                    scaling,
                );
            }
            det = det
                * self.row_scaling(
                    pivot_row,
                    scaling,
                );
            // Reduce all other rows
            for row in 1..=n {
                if row != pivot_row {
                    if inv {
                        _ = inv_m.row_static_add(
                            row,
                            pivot_row,
                            self.el(row, col),
                        );
                    }
                    det = det
                        * self.row_static_add(
                            row,
                            pivot_row,
                            K::default() - self.el(row, col),
                        );
                }
            }
        }

        (det, pivot_row, inv_m)
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let mut to_return = self.clone();
        let (_, _, _) = to_return.gauss_red_det_rank(false);
        to_return
    }

    pub fn determinant(&self) -> K {
        let (m, n) = self.size();
        assert!(m == n, "Not a square matrix. Can't calculate determinant.");
        let mut to_return = self.clone();
        let (det, _, _) = to_return.gauss_red_det_rank(false);
        det
    }

    pub fn rank(&self) -> usize {
        let mut to_return = self.clone();
        let (_, rank, _) = to_return.gauss_red_det_rank(false);
        rank
    }

    pub fn diag(diag: Vec<K>) -> Matrix<K> {
        let n = diag.len();
        assert!(n > 0, "Insuficient data");
        let mut to_return = Matrix::<K>::zero(n, n);
        for i in 1..=n {
            to_return.set(i, i, diag[i - 1]);
        }
        to_return
    }

    pub fn identity(n: usize) -> Matrix<K> {
        Matrix::<K>::diag(vec![K::one(); n])
    }

    pub fn inverse(&self) -> Matrix<K> {
        let (m, n) = self.size();
        assert!(m == n, "Not a square matrix. Can't calculate inverse.");
        let mut to_alg = self.clone();
        let (_, rank, inv) = to_alg.gauss_red_det_rank(true);
        assert!(rank == n, "Matrix is not invertible");

        inv
    }
    pub fn projection(
        fov: f64,
        ratio: f64,
        near: f64,
        far: f64,
    ) -> Matrix<f64> {
        let mut to_return = Matrix::<f64>::zero(4, 4);
        let f = 1. / f64::tan(fov / 2.);
        to_return.set(1, 1, f / ratio);
        to_return.set(2, 2, f);
        to_return.set(3, 3, -(far) / (near - far));
        to_return.set(4, 3, 1.0);
        to_return.set(3, 4, near * far / (near - far));
        to_return
    }
}

impl<K> fmt::Display for Matrix<K>
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
        + Conj
        + Neg,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (m, n) = self.size();
        write!(f, "[")?;
        for j in 1..=n {
            write!(f, "[")?;

            for i in 1..=m {
                write!(f, "{}", self.el(i, j))?;
                if i != m {
                    write![f, ","]?;
                }
            }
            write!(f, "]")?;
            if j != n {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}
