mod complex;
mod matrix;

use crate::complex::ComplexNumber;
use crate::matrix::Matrix;

fn main() {
    {
    //Complex number test
    let x = ComplexNumber::Cartesian {re: 1.0, im: -1.0};
    println!("{} {}",x.clone()*x.conj(), x.clone()*x.inv());
    }
    {
        let m1 = Matrix::<f64>::new(vec![1.0,2.0,3.0,4.0],2,2);
        for i in 1..=2{
            for j in 1..=2{
                println!("{} {} {}", i, j, m1.el(i,j));
            }
        }
        let (n,m) = m1.size();
        println!("{} {}", m, n);

        let m2 = Matrix::<f64>::new(vec![5.0,0.0,8.0,0.0],2,2);
        let m3 = m1.mlt(m2);
        for i in 1..=2{
            for j in 1..=2{
                println!("{} {} {}", i, j, m3.el(i,j));
            }
        }
    }
}
