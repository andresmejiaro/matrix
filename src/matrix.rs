use std::ops::{Add,Sub,Mul,Div};

use crate::complex::ComplexNumber;

#[derive(PartialEq,Clone)]
pub struct Matrix<K>
where 
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> 
    + Div<Output = K>  + PartialEq + Copy+ Default{
        size: (usize, usize),
        elements: Vec<K>
}

impl<K> Matrix<K>
where 
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> 
    + Div<Output = K>  + PartialEq + Copy + Default{
    pub fn new(elements: Vec<K>, n: usize, m: usize)-> Matrix<K>{
        assert!(elements.len() == n*m, "The number of elements does not match!");

        Matrix {size: (n,m), elements}    
    }

    pub fn size(&self)-> (usize, usize){
        self.size
    }

    pub fn el(&self,i: usize, j:usize) -> K{
        let (n,m) = self.size;
        assert!(i <= n && j<=m);
        self.elements[i - 1 + (j - 1)*n]
    }

    pub fn set(&mut self,i: usize, j:usize, val : K){
        let (n,m) = self.size;
        assert!(i <= n && j<=m);
        self.elements[i - 1 + (j - 1)*n] = val;
    }

    pub fn tr(&self) -> Matrix<K>{
        let (n,m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m*n], m ,n);
        for i in 1..=n{
            for j in 1..=n{
                to_return.set(i,j, self.el(j,i));
            }
        }
        to_return
    }

    fn zero(m: usize, n: usize)-> Matrix<K>{
        Matrix::<K>::new(vec![K::default(); m*n], m ,n)
    }

    fn add(&self, other: Matrix<K>)-> Matrix<K>{
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let (n,m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m*n], m ,n);
        for i in 1..=n{
            for j in 1..=n{
                to_return.set(i,j, self.el(i,j) + other.el(i,j));
            }
        }
        to_return
    }

    fn scl(&self, scaling: K)-> Matrix<K>{
        let (n,m) = self.size;
        let mut to_return = Matrix::zero(m ,n);
        for i in 1..=n{
            for j in 1..=n{
                to_return.set(i,j, scaling * self.el(i,j));
            }
        }
        to_return
    }

    fn sub(&self, other: Matrix<K>)-> Matrix<K>{
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let (n,m) = self.size;
        let mut to_return = Matrix::<K>::new(vec![K::default(); m*n], m ,n);
        for i in 1..=n{
            for j in 1..=n{
                to_return.set(i,j, self.el(i,j) - other.el(i,j));
            }
        }
        to_return
    }

    fn append_horizontal(&self, other: &Matrix<K>)->Matrix<K>{
        let (n1,m1) = self.size;
        let (n2,m2) = other.size;
        assert_eq!(n1,n2, "Sizes not compatible for joining");
        Matrix::new(self.elements.iter().cloned().
            chain(other.elements.iter().cloned()).collect(),
             n1 , m1+ m2)
    }

    pub fn mlt(&self, other: Matrix<K>)-> Matrix<K>{
        let (n,p1) = self.size();
        let (p2,m) = other.size();
        assert_eq!(p1,p2,"Size for multiplication not compatible");
        let mut newv: Vec<K> = (0..m*n).map(|_| K::default()).collect();
        for i in 0..n {
            for j in 0..m{
                for w in 0..p1{
                    newv[j + i*n] = newv[j + i*n] + self.el(i+1,w+1) * other.el(w+1,j+1); 
                }
            }
        }
        Matrix::<K>::new(newv,n,m)
    }


}


impl Matrix<ComplexNumber> {
    pub fn adjunct(&self) -> Matrix<K>{
        let (n,m) = self.size;
        let mut to_return = Matrix::<ComplexNumber>::
            new(vec![ComplexNumber::default(); m*n], m ,n);
        for i in 1..=n{
            for j in 1..=n{
                to_return.set(i,j, self.el(j,i).conj());
            }
        }
        to_return
    }

    
}

pub struct Vector<K>
where 
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> 
    + Div<Output = K>  + PartialEq + Copy+ Default{
        size: usize,
        matrix: Matrix<K>
}



impl<K>Vector<K>
where 
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> 
    + Div<Output = K>  + PartialEq + Copy + Default{
    pub fn new(elements: Vec<K>, n: usize )-> Vector<K>{
        assert!(elements.len() == n, "The number of elements does not match!");

        Vector {size: n, matrix : Matrix::new(elements, n , 1)}    
    }

    pub fn size(&self)-> usize{
        self.size
    }

    pub fn el(&self,i: usize) -> K{
        let n = self.size;
        assert!(i <= n);
        self.matrix.el(i,1)
    }

    pub fn set(&mut self,i: usize, val : K){
        let n= self.size;
        assert!(i <= n);
        self.matrix.set(i,1,val);
    }

    fn zero(n: usize)-> Vector<K>{
        Vector::<K>::new(vec![K::default(); n], n)
    }

    fn add(&self, other: Vector<K>)-> Vector<K>{
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let n = self.size;
        let mut to_return = Vector::<K>::new(self.matrix.add(other.matrix).elements,n);
        to_return
    }

    fn scl(&self, scaling: K)-> Vector<K>{
        let n= self.size;
        Vector::new(self.matrix.scl(scaling).elements, n)
    }

    fn sub(&self, other: Vector<K>)-> Vector<K>{
        assert_eq!(self.size(), other.size(), "Size doesn't match");
        let n = self.size;
        Vector::<K>::new(self.matrix.sub(other.matrix).elements, n)
    }
    
    fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K>{
        assert!(u.is_empty() || coefs.is_empty(), "Empty data for lineal combination");
        assert_eq!(u.len(), coefs.len(),
         "Number of vectors and coefficients doesn't match for linear combination");
        let mut to_return = Vector::<K>::zero(u[0].size);
        for (v,s) in u.iter().zip(coefs.iter()){
            to_return = to_return.add(v.scl(*s));
        }
        to_return
    }

}

impl Vector<f32>{
    fn lerp(u: Vector<f32>, v: Vector<f32>, t: f32)-> Vector<f32>{
        Vector::<f32>::linear_combination(&[u,v],&[1.0-t,t] )
    }
}

impl Vector<f64>{
    fn lerp(u: Vector<f64>, v: Vector<f64>, t: f64)-> Vector<f64>{
        Vector::<f64>::linear_combination(&[u,v],&[1.0-t,t] )
    }
}

impl Vector<ComplexNumber>{
    fn lerp(u: Vector<ComplexNumber>, v: Vector<ComplexNumber>, t: ComplexNumber)-> Vector<ComplexNumber>{
        Vector::<ComplexNumber>::linear_combination(&[u,v],
            &[ComplexNumber::Cartesian { re: (1.0), im: (0.0) } - t, t] )
    }
}