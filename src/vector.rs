use crate::errors::LinAlgError;
use crate::matrix::Matrix;
use crate::traits::{Field, Tf64};
use core::fmt;

// Define `Vector` struct to represent vectors with elements of type `K`, where `K` implements the `Field` trait
pub struct Vector<K>
where
    K: Field,
{
    size: usize,
    pub matrix: Matrix<K>,
}

// Implement vector functionalities for type `K` where `K` implements the `Field` trait
impl<K> Vector<K>
where
    K: Field,
{
    // Constructor for `Vector` that wraps a `Matrix` for its internal representation
    pub fn new(
        elements: Vec<K>,
    ) -> Result<Vector<K>, LinAlgError> {
        if elements.len() == 0 {
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
    // Retrieve the size (number of elements) of the vector
    pub fn size(&self) -> usize {
        self.size
    }
    // Accessor method to get the underlying matrix representation of the vector
    pub fn matrix(&self) -> &Matrix<K> {
        &self.matrix
    }
    // Access a specific element of the vector, with bounds checking
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

    // Create a zero vector of specified size
    pub fn zero(
        n: usize,
    ) -> Result<Vector<K>, LinAlgError> {
        Vector::<K>::new(vec![K::default(); n])
    }

    // Create a vector filled with ones of specified size
    pub fn ones(
        n: usize,
    ) -> Result<Vector<K>, LinAlgError> {
        Vector::<K>::new(vec![K::one(); n])
    }

    // Add two vectors of the same size
    pub fn add(
        &self,
        other: &Vector<K>,
    ) -> Result<Vector<K>, LinAlgError> {
        if self.size() != other.size() {
            return Err(
                LinAlgError::OperationNonConforming {
                    operation: "sum".to_string(),
                },
            );
        }
        let to_return = Vector::<K>::new(
            self.matrix.add(&other.matrix)?.elements,
        );
        to_return
    }

    // Scale the vector by a factor
    pub fn scl(
        &self,
        scaling: K,
    ) -> Result<Vector<K>, LinAlgError> {
        Vector::new(self.matrix.scl(scaling)?.elements)
    }

    // Subtract one vector from another of the same size
    pub fn sub(
        &self,
        other: &Vector<K>,
    ) -> Result<Vector<K>, LinAlgError> {
        if self.size() != other.size() {
            return Err(
                LinAlgError::OperationNonConforming {
                    operation: "sub".to_string(),
                },
            );
        }
        Vector::<K>::new(
            self.matrix.sub(&other.matrix)?.elements,
        )
    }

    // Compute a linear combination of vectors
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

    // Linearly interpolate between two vectors
    pub fn lerp(
        u: &Vector<K>,
        v: &Vector<K>,
        t: K,
    ) -> Result<Vector<K>, LinAlgError> {
        Vector::<K>::linear_combination(
            &[u, v],
            &[K::one() - t, t],
        )
    }

    // Compute the dot product of two vectors
    pub fn dot(
        &self,
        other: &Vector<K>,
    ) -> Result<K, LinAlgError> {
        if self.size() != other.size() {
            return Err(LinAlgError::BuildNonconforming {
                expected: self.size(),
                recieved: other.size(),
            });
        }
        if self.size() == 0 {
            return Ok(K::default());
        } else {
            let m1 =
                self.matrix.adj()?.mlt(&other.matrix)?;
            Ok(m1.el(1, 1)?)
        }
    }

    // Compute the Euclidean norm (magnitude) of the vector
    pub fn norm(&self) -> f64 {
        self.dot(self).unwrap().tf64().powf(0.5)
    }

    // Compute the \(L_1\) norm (sum of absolute values) of the vector
    pub fn norm_1(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.matrix.elements {
            x += i.norm();
        }
        x
    }

    // Compute the infinity norm (maximum absolute value) of the vector
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

    // Calculate the cosine of the angle between two vectors
    pub fn angle_cos(
        &self,
        other: &Self,
    ) -> Result<f64, LinAlgError> {
        if self.norm() == 0. || other.norm() == 0. {
            return Err(LinAlgError::SinglarMatrix);
        }
        let dotp = self.dot(other)?;
        Ok(dotp.tf64() / (self.norm() * other.norm()))
    }

    // Compute the cross product of two 3-dimensional vectors
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
        let c1 = self.el(2)? * other.el(3)?
            - other.el(2)? * self.el(3)?;
        let c2 = (K::default() - self.el(1)?)
            * other.el(3)?
            + self.el(3)? * other.el(1)?;
        let c3 = self.el(1)? * other.el(2)?
            - self.el(2)? * other.el(1)?;
        Vector::new(vec![c1, c2, c3])
    }

    // Set the value of a specific element in the vector, with bounds checking
    pub fn set(
        &mut self,
        el: usize,
        val: K,
    ) -> Result<(), LinAlgError> {
        let m = self.size();
        if el > m {
            return Err(LinAlgError::OutofBoundsVector {
                size: m,
                recieved: el,
            });
        }
        self.matrix.set(el, 1, val)?;
        Ok(())
    }
}

// Specialize `Vector<f64>` for operations involving normalization
impl Vector<f64> {
    // Normalize the vector and return the normalized vector along with the mean and standard deviation used for normalization
    pub fn normalize_vec(
        &self,
    ) -> Result<(Vector<f64>, f64, f64), LinAlgError> {
        let (to_ret, norm_coef) =
            self.matrix.normalize_cols()?;
        Ok((
            to_ret.column_extract(1)?,
            norm_coef.el(1, 1)?,
            norm_coef.el(2, 1)?,
        ))
    }
}

// Implement the Display trait for `Vector` to enable custom formatting
impl<K> fmt::Display for Vector<K>
where
    K: Field,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
