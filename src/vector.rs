use crate::errors::LinAlgError;
use crate::matrix::Matrix;
use crate::traits::{Field, Tf64};
use core::fmt;

pub struct Vector<K>
where
    K: Field,
{
    size: usize,
    matrix: Matrix<K>,
}

impl<K> Vector<K>
where
    K: Field,
{
    pub fn new(elements: Vec<K>) -> Result<Vector<K>, LinAlgError> {
        if elements.len() > 0 {
            return Err(LinAlgError::BuildNonconforming {
                expected: 1,
                recieved: 0,
            });
        }
        let n = elements.len();

        let v = Vector {
            size: n,
            matrix: Matrix::new(elements, n, 1)?,
        };
        Ok(v)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn matrix(&self) -> &Matrix<K> {
        &self.matrix
    }

    pub fn el(&self, i: usize) -> Result<K, LinAlgError> {
        let n = self.size;
        if i > n {
            return Err(LinAlgError::OutofBoundsVector {
                size: self.size,
                recieved: i,
            });
        }
        Ok(self.matrix.el(i, 1)?)
    }

    pub fn zero(n: usize) -> Result<Vector<K>, LinAlgError> {
        Vector::<K>::new(vec![K::default(); n])
    }

    pub fn add(&self, other: &Vector<K>) -> Result<Vector<K>, LinAlgError> {
        if self.size() != other.size() {
            return Err(LinAlgError::OperationNonConforming {
                operation: "sum".to_string(),
            });
        }
        let to_return =
            Vector::<K>::new(self.matrix.add(&other.matrix)?.elements);
        to_return
    }

    pub fn scl(&self, scaling: K) -> Result<Vector<K>, LinAlgError> {
        Vector::new(self.matrix.scl(scaling)?.elements)
    }

    pub fn sub(&self, other: &Vector<K>) -> Result<Vector<K>, LinAlgError> {
        if self.size() != other.size() {
            return Err(LinAlgError::OperationNonConforming {
                operation: "sub".to_string(),
            });
        }
        Vector::<K>::new(self.matrix.sub(&other.matrix)?.elements)
    }

    pub fn linear_combination(
        u: &[&Vector<K>],
        coefs: &[K],
    ) -> Result<Vector<K>, LinAlgError> {
        if u.len() == 0 || coefs.len() == 0 {
            return Err(LinAlgError::EmptyArgs);
        }
        if u.len() != coefs.len() {
            return Err(LinAlgError::BuildNonconforming {
                expected: u.len(),
                recieved: coefs.len(),
            });
        }
        let mut to_return = Vector::<K>::zero(u[0].size)?;
        for (v, s) in u.iter().zip(coefs.iter()) {
            to_return = to_return.add(&v.scl(*s)?)?;
        }
        Ok(to_return)
    }

    pub fn lerp(
        u: &Vector<K>,
        v: &Vector<K>,
        t: K,
    ) -> Result<Vector<K>, LinAlgError> {
        Vector::<K>::linear_combination(&[u, v], &[K::one() - t, t])
    }

    pub fn dot(&self, other: &Vector<K>) -> Result<K, LinAlgError> {
        if self.size() != other.size() {
            return Err(LinAlgError::BuildNonconforming {
                expected: self.size(),
                recieved: other.size(),
            });
        }
        if self.size() == 0 {
            return Ok(K::default());
        } else {
            let m1 = self.matrix.adj()?.mlt(&other.matrix)?;
            Ok(m1.el(1, 1)?)
        }
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).unwrap().tf64().powf(0.5)
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

    pub fn angle_cos(&self, other: &Self) -> Result<f64, LinAlgError> {
        if self.norm() == 0. || other.norm() == 0. {
            return Err(LinAlgError::SinglarMatrix);
        }
        let dotp = self.dot(other)?;
        Ok(dotp.tf64() / (self.norm() * other.norm()))
    }

    pub fn cross_product(
        &self,
        other: &Self,
    ) -> Result<Vector<K>, LinAlgError> {
        if self.size() != 3 {
            return Err(LinAlgError::BuildNonconforming {
                expected: 3,
                recieved: self.size(),
            });
        }
        if other.size() != 3 {
            return Err(LinAlgError::BuildNonconforming {
                expected: 3,
                recieved: other.size(),
            });
        }
        let c1 = self.el(2)? * other.el(3)? - other.el(2)? * self.el(3)?;
        let c2 = (K::default() - self.el(1)?) * other.el(3)?
            + self.el(3)? * other.el(1)?;
        let c3 = self.el(1)? * other.el(2)? - self.el(2)? * other.el(1)?;
        Vector::new(vec![c1, c2, c3])
    }
}

impl<K> fmt::Display for Vector<K>
where
    K: Field,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.size();
        write!(f, "[")?;
        for j in 1..=n {
            write!(f, "{}", self.matrix.el(j, 1).unwrap())?;
            if j != n {
                write![f, ","]?;
            }
        }
        write!(f, "]")
    }
}
