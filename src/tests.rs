use crate::complex::ComplexNumber;
use crate::errors::LinAlgError;
use crate::matrix::Matrix;
use crate::traits::Conj;
use crate::vector::Vector;

// Defines a series of test functions to validate matrix and vector operations 
// and their interactions, especially focusing on error handling and complex number manipulations.
pub fn old_test() -> Result<(), LinAlgError> {
    {
        println!(
            "{}",
            Matrix::<f64>::projection(
                80. * 3.14 / 180.,
                1.777,
                0.1,
                1000.
            )?
        );
    }

    {
        //Complex number test
        println!("Complex number printing test");
        let x =
            ComplexNumber::Cartesian { re: 1.0, im: -1.0 };
        println!(
            "{} {}",
            x.clone() * x.conj(),
            x.clone() * x.inv()
        );
        println!("Its real part is {}", x.re());
        println!("Its complex part is {}", x.im());
    }
    // Basic Printing Test
    {
        println!("Matrix number printing test");
        let m1 = Matrix::<f64>::new(
            vec![1.0, 2.0, 3.0, 4.0],
            2,
            2,
        )?;
        println!("m1 = {}", m1);
        let det = m1.determinant()?;
        println!("det {} m1 {}", det, m1);
        let m2 = Matrix::<f64>::new(
            vec![5.0, 0.0, 8.0, 0.0],
            2,
            2,
        )?;
        println!("m2 = {}", m2);
        println!("The trace of m1 {}", m1.trace()?);
        println!(
            "Concatenating them {}",
            m1.append_horizontal(&m2)?
        );
        println!("Complex number printing test");
        let m1 = Matrix::<ComplexNumber>::new(
            vec![
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (1.0),
                },
                ComplexNumber::Cartesian {
                    re: (-1.0),
                    im: (1.0),
                },
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (1.0),
                },
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (1.0),
                },
            ],
            2,
            2,
        )?;
        println!("m1 = {}", m1);
        let m2 = Matrix::<ComplexNumber>::new(
            vec![
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (1.0),
                },
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (-1.0),
                },
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (1.0),
                },
                ComplexNumber::Cartesian {
                    re: (1.0),
                    im: (1.0),
                },
            ],
            2,
            2,
        )?;
        println!("m2 = {}", m2);
        let m3 = m1.mlt(&m2)?;
        println!("m3 = {}", m3);
        println!("compare the transpose and the adjunct");
        println!("{} {}", m1.tr()?, m1.adj()?);
    }
    {
        println!("Basic Operations with Vectors");
        let v1 = Vector::<f64>::new(vec![1., 2., 3.])?;
        println!("A vector {}", v1);
        println!("Scaling by 2.3 {}", v1.scl(2.3)?);
        println!(
            "Getting the second element with an internal method {}",
            v1.el(2)?
        );
        println!(
            "Creating a 100 sized zero real vector {}",
            Vector::<f64>::zero(100)?
        );
        println!(
            "Creating a 100 sized zero complex vector {}",
            Vector::<ComplexNumber>::zero(100)?
        );
        let v2 = Vector::<f64>::new(vec![4., 6., 7.])?;
        println!("Another vector {}", v2);
        println!(
            "Adding them using both as the \"left\" vector {} {}",
            v1.add(&v2)?,
            v2.add(&v1)?
        );
        println!(
            "Substracting them using both as the \"left\" vector {} {}",
            v1.sub(&v2)?,
            v2.sub(&v1)?
        );
        println!(
            "Creating a lerp manually between v1 and v2 0.7 and 0.3 {}",
            v1.scl(0.7)?.add(&v2.scl(0.3)?)?
        );
        println!("Creating a lerp using linear combination between v1 and v2 0.7 and 0.3 {}", 
            Vector::linear_combination(&[&v1,&v2], &[0.7,0.3])?);
        println!("Creating a lerp using lerp function between v1 and v2 0.7 and 0.3 {}", 
            Vector::<f64>::lerp(&v1,&v2, 0.7)?);
        let v3 = v1.cross_product(&v2)?;
        println!(
            "the cross product between those two is {}",
            v3
        );
        println!(
            "Look ma im orthogonal {} {}",
            v3.dot(&v1)?,
            v3.dot(&v2)?
        );
        println!("Same with f32");
        let v1 = Vector::<f32>::new(vec![1., 2., 3.])?;
        let v2 = Vector::<f32>::new(vec![4., 6., 7.])?;
        println!("Creating a lerp using lerp function between v1 and v2 0.7 and 0.3 {}", 
            Vector::<f32>::lerp(&v1,&v2, 0.7)?);
        let v3 = v1.cross_product(&v2)?;
        println!(
            "the cross product between those two is {}",
            v3
        );
        println!(
            "Look ma im orthogonal {} {}",
            v3.dot(&v1)?,
            v3.dot(&v2)?
        );
    }
    {
        println!(
            "Basic Operations with Matrices and Complex"
        );
        let m1 = Matrix::<ComplexNumber>::new(
            vec![
                ComplexNumber::n(1.0, 0.),
                ComplexNumber::n(0., 1.0),
                ComplexNumber::n(-1., 1.),
                ComplexNumber::n(2., -1.),
                ComplexNumber::n(-2., 2.),
                ComplexNumber::n(-2., 0.),
            ],
            3,
            2,
        )?;
        println!("A complex matrix {}", m1);
        println!(
            "Scaling by 2.3 + 0.3i {}",
            m1.scl(ComplexNumber::n(2.3, 0.3))?
        );
        println!(
            "Creating a 10 x 5 sized zero real vector {}",
            Matrix::<ComplexNumber>::zero(10, 5)?
        );
        let m2 = Matrix::<ComplexNumber>::new(
            vec![
                ComplexNumber::n(3.0, 4.),
                ComplexNumber::n(0.32, 3.0),
                ComplexNumber::n(-1.4, 14.),
                ComplexNumber::n(24., -12.),
                ComplexNumber::n(-2., 2.),
                ComplexNumber::n(-32.3, 0.3),
            ],
            3,
            2,
        )?;
        println!("Another matrix {}", m2);
        println!(
            "Adding them using both as the \"left\" vector {} {}",
            m1.add(&m2)?,
            m2.add(&m1)?
        );
        println!(
            "Substracting them using both as the \"left\" vector {} {}",
            m1.sub(&m2)?,
            m2.sub(&m1)?
        );
        println!(
            "Creating a linear combination between m1,m2 2 + 31 , - 6i {}",
            Matrix::linear_combination(
                &[&m1, &m2],
                &[ComplexNumber::n(2., 3.), ComplexNumber::n(3., 6.)]
            )?
        );
    }
    {
        //Vectorial Norms

        let v1 = Vector::<f64>::new(vec![1., 2., 3.])?;
        let v2 = Vector::<f64>::new(vec![4., 6., 7.])?;
        println!(
            "The product between v1 = {} and v2 = {} is {}",
            v1,
            v2,
            v1.dot(&v2)?
        );
        println!(
            "The product between v1 = {} and v1 = {} is {}",
            v1,
            v1,
            v1.dot(&v1)?
        );
        println!("The (L2) norm of v1 is {}", v1.norm());
        println!("The (L1) norm of v1 is {}", v1.norm_1());
        println!(
            "The (Linf) norm of v1 is {}",
            v1.norm_inf()
        );
        println!(
            "The cosine between v1 and v2 is {}",
            v1.angle_cos(&v2)?
        );

        // Same example f32
        println!("Same example f32");
        let v1 = Vector::<f32>::new(vec![1., 2., 3.])?;
        let v2 = Vector::<f32>::new(vec![4., 6., 7.])?;
        println!(
            "The product between v1 = {} and v2 = {} is {}",
            v1,
            v2,
            v1.dot(&v2)?
        );
        println!(
            "The product between v1 = {} and v1 = {} is {}",
            v1,
            v1,
            v1.dot(&v1)?
        );
        println!("The (L2) norm of v1 is {}", v1.norm());
        println!("The (L1) norm of v1 is {}", v1.norm_1());
        println!(
            "The (Linf) norm of v1 is {}",
            v1.norm_inf()
        );
        println!(
            "The cosine between v1 and v2 is {}",
            v1.angle_cos(&v2)?
        );

        let v1 = Vector::<ComplexNumber>::new(vec![
            ComplexNumber::n(1., 2.),
            ComplexNumber::n(3., 4.),
            ComplexNumber::n(4., -1.),
        ])?;
        let v2 = v1.scl(ComplexNumber::n(-3., 4.))?;
        println!(
            "The product between v1 = {} and v2 = {} is {}",
            v1,
            v2,
            v1.dot(&v2)?
        );
        println!(
            "The product between v1 = {} and v1 = {} is {}",
            v1,
            v1,
            v1.dot(&v1)?
        );
        println!("The (L2) norm of v1 is {}", v1.norm());
        println!("The (L1) norm of v1 is {}", v1.norm_1());
        println!(
            "The (Linf) norm of v1 is {}",
            v1.norm_inf()
        );
        println!("Now that we are here lets larp complex!");
        println!(
            "{}",
            Vector::<ComplexNumber>::lerp(
                &v1,
                &v2,
                ComplexNumber::n(0.7, 0.)
            )?
        );
    }
    Ok(())
}
// Prints a line for visual separation of test sections
pub fn big_line() {
    println!("----------------------------------------------------------");
}
// Tests basic arithmetic operations with vectors, demonstrating addition,
// subtraction, and scaling
pub fn ex00_test() -> Result<(), LinAlgError> {
    println!("Subject example");
    let mut u = Vector::<f64>::new(vec![2., 3.])?;
    let v = Vector::<f64>::new(vec![5., 7.])?;
    u = u.add(&v)?;
    println!("{}", &u);
    println!("// [7.0]");
    println!("// [10.0]");
    let mut u = Vector::<f64>::new(vec![2., 3.])?;
    let v = Vector::<f64>::new(vec![5., 7.])?;
    u = u.sub(&v)?;
    println!("{}", u);
    println!("// [-3.0]");
    println!("// [-4.0]");
    let mut u = Vector::<f64>::new(vec![2., 3.])?;
    u = u.scl(2.)?;
    println!("{}", u);
    println!("// [4.0]");
    println!("// [6.0]");
    let u = Matrix::<f64>::new(vec![1., 2., 3., 4.], 2, 2)?;
    let v = Matrix::new(vec![7., 4., -2., 2.], 2, 2)?;
    println!("{}", u.add(&v)?);
    println!("// [8.0, 6.0]");
    println!("// [1.0, 6.0]");
    let mut u =
        Matrix::<f64>::new(vec![1., 2., 3., 4.], 2, 2)?;
    let v =
        Matrix::<f64>::new(vec![7., 4., -2., 2.], 2, 2)?;
    u = u.sub(&v)?;
    println!("{}", u);
    println!("// [-6.0, -2.0]");
    println!("// [5.0, 2.0]");
    let mut u = Matrix::new(vec![1., 2., 3., 4.], 2, 2)?;
    u = u.scl(2.)?;
    println!("{}", u);
    println!("// [2.0, 4.0]");
    println!("// [6.0, 8.0]");
    Ok(())
}

// Demonstrates linear combination of vectors and the flexibility of the
// implemented operations with both floating point and complex numbers
pub fn ex01_test() -> Result<(), LinAlgError> {
    let e1 = Vector::<f64>::new(vec![1., 0., 0.])?;
    let e2 = Vector::<f64>::new(vec![0., 1., 0.])?;
    let e3 = Vector::<f64>::new(vec![0., 0., 1.])?;
    let v1 = Vector::<f64>::new(vec![1., 2., 3.])?;
    let v2 = Vector::<f64>::new(vec![0., 10., -100.])?;
    println!(
        "{}",
        Vector::<f64>::linear_combination(
            &[&e1, &e2, &e3],
            &[10., -2., 0.5]
        )?
    );
    println!("// [10.]");
    println!("// [-2.]");
    println!("// [0.5]");
    println!(
        "{}",
        Vector::<f64>::linear_combination(
            &[&v1, &v2],
            &[10., -2.]
        )?
    );
    println!("// [10.]");
    println!("// [0.]");
    println!("// [230.]");
    Ok(())
}

// Tests linear interpolation (lerp) between vectors and matrices, showcasing
//the ability to smoothly transition between two entities
pub fn ex02_test() -> Result<(), LinAlgError> {
    println!(
        "{}",
        Vector::<f64>::lerp(
            &Vector::<f64>::new(vec![0.])?,
            &Vector::<f64>::new(vec![1.])?,
            0.
        )?
    );
    println!("// 0.0");
    println!(
        "{}",
        Vector::<f64>::lerp(
            &Vector::<f64>::new(vec![0.])?,
            &Vector::<f64>::new(vec![1.])?,
            1.
        )?
    );
    println!("// 1.0");
    println!(
        "{}",
        Vector::<f64>::lerp(
            &Vector::<f64>::new(vec![0.])?,
            &Vector::<f64>::new(vec![1.])?,
            0.5
        )?
    );

    println!("// 0.5");
    println!(
        "{}",
        Vector::<f32>::lerp(
            &Vector::<f32>::new(vec![21.])?,
            &Vector::<f32>::new(vec![42.])?,
            0.3
        )?
    );
    println!("// 27.3");
    println!(
        "{}",
        Vector::<f32>::lerp(
            &Vector::new(vec![2., 1.])?,
            &Vector::new(vec![4., 2.])?,
            0.3
        )?
    );
    println!("// [2.6]");
    println!("// [1.3]");
    println!(
        "{}",
        Matrix::<f32>::lerp(
            &Matrix::<f32>::new(
                vec![2., 1., 3., 4.],
                2,
                2
            )?,
            &Matrix::<f32>::new(
                vec![20., 10., 30., 40.],
                2,
                2
            )?,
            0.5
        )?
    );
    println!("// [[11., 5.5]");
    println!("// [16.5, 22.]]");
    Ok(())
}

// Validates the dot product operation between vectors, ensuring correct
// computation for both real and complex numbers
pub fn ex03_test() -> Result<(), LinAlgError> {
    let u = Vector::<f32>::new(vec![0., 0.])?;
    let v = Vector::<f32>::new(vec![1., 1.])?;
    println!("{}", u.dot(&v)?);
    println!("// 0.0");
    let u = Vector::new(vec![1., 1.])?;
    let v = Vector::new(vec![1., 1.])?;
    println!("{}", u.dot(&v)?);
    println!("// 2.0");
    let u = Vector::new(vec![-1., 6.])?;
    let v = Vector::new(vec![3., 2.])?;
    println!("{}", u.dot(&v)?);
    println!("// 9.0");
    Ok(())
}
// Focuses on vector norms, testing L1, L2 (Euclidean), and Linf (infinity)
// norms to verify their correct computation
pub fn ex04_test() -> Result<(), LinAlgError> {
    let u = Vector::new(vec![0., 0., 0.])?;
    println!(
        "{}, {}, {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    println!("// 0.0, 0.0, 0.0");
    let u = Vector::new(vec![1., 2., 3.])?;
    println!(
        "{}, {}, {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    println!("// 6.0, 3.74165738, 3.0");
    let u = Vector::new(vec![-1., -2.])?;
    println!(
        "{}, {}, {}",
        u.norm_1(),
        u.norm(),
        u.norm_inf()
    );
    println!("// 3.0, 2.236067977, 2.0");
    Ok(())
}

// Tests the cosine of the angle between vectors, validating the calculation
// for vectors in various orientations and magnitudes.
pub fn ex05_test() -> Result<(), LinAlgError> {
    let u = Vector::new(vec![1., 0.])?;
    let v = Vector::new(vec![1., 0.])?;
    println!("{}", Vector::angle_cos(&u, &v)?);
    println!("// 1.0");
    let u = Vector::new(vec![1., 0.])?;
    let v = Vector::new(vec![0., 1.])?;
    println!("{}", Vector::angle_cos(&u, &v)?);
    println!("// 0.0");
    let u = Vector::new(vec![-1., 1.])?;
    let v = Vector::new(vec![1., -1.])?;
    println!("{}", Vector::angle_cos(&u, &v)?);
    println!("// -1.0");
    let u = Vector::new(vec![2., 1.])?;
    let v = Vector::new(vec![4., 2.])?;
    println!("{}", Vector::angle_cos(&u, &v)?);
    println!("// 1.0");
    let u = Vector::new(vec![1., 2., 3.])?;
    let v = Vector::new(vec![4., 5., 6.])?;
    println!("{}", Vector::angle_cos(&u, &v)?);
    println!("// 0.974631846");
    Ok(())
}
// Demonstrates the cross product between vectors, showcasing the orthogonality
// and directionality aspects of the cross product operation.
pub fn ex06_test() -> Result<(), LinAlgError> {
    let u = Vector::new(vec![0., 0., 1.])?;
    let v = Vector::new(vec![1., 0., 0.])?;
    println!("{}", Vector::cross_product(&u, &v)?);
    println!("// [0.]");
    println!("// [1.]");
    println!("// [0.]");
    let u = Vector::new(vec![1., 2., 3.])?;
    let v = Vector::new(vec![4., 5., 6.])?;
    println!("{}", Vector::cross_product(&u, &v)?);
    println!("// [-3.]");
    println!("// [6.]");
    println!("// [-3.]");
    let u = Vector::new(vec![4., 2., -3.])?;
    let v = Vector::new(vec![-2., -5., 16.])?;
    println!("{}", Vector::cross_product(&u, &v)?);
    println!("// [17.]");
    println!("// [-58.]");
    println!("// [-16.]");
    Ok(())
}
// Validates matrix-vector and matrix-matrix multiplication, highlighting the
//correctness of these operations across different scenarios.
pub fn ex07_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(vec![1., 0., 0., 1.], 2, 2)?;
    let v = Vector::new(vec![4., 2.])?;
    println!("{}", u.mul_vec(&v)?);
    println!("// [4.]");
    println!("// [2.]");
    let u = Matrix::new(vec![2., 0., 0., 2.], 2, 2)?;
    let v = Vector::new(vec![4., 2.])?;
    println!("{}", u.mul_vec(&v)?);
    println!("// [8.]");
    println!("// [4.]");
    let u = Matrix::new(vec![2., -2., -2., 2.], 2, 2)?;
    let v = Vector::new(vec![4., 2.])?;
    println!("{}", u.mul_vec(&v)?);
    println!("// [4.]");
    println!("// [-4.]");
    let u = Matrix::new(vec![1., 0., 0., 1.], 2, 2)?;
    let v = Matrix::new(vec![1., 0., 0., 1.], 2, 2)?;
    println!("{}", u.mul_mat(&v)?);
    println!("// [1.,0.]");
    println!("// [0.,1.]");
    let u = Matrix::new(vec![1., 0., 0., 1.], 2, 2)?;
    let v = Matrix::new(vec![2., 4., 1., 2.], 2, 2)?;
    println!("{}", u.mul_mat(&v)?);
    println!("// [2.,1.]");
    println!("// [4.,2.]");
    let u = Matrix::new(vec![3., 6., -5., 8.], 2, 2)?;
    let v = Matrix::new(vec![2., 4., 1., 2.], 2, 2)?;
    println!("{}", u.mul_mat(&v)?);
    println!("// [-14.,7.]");
    println!("// [44.,22.]");
    Ok(())
}
// Tests the trace operation on matrices, ensuring the sum of diagonal elements
//is correctly computed for square matrices.
pub fn ex08_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(vec![1., 0., 0., 1.], 2, 2)?;
    println!("{}", u.trace()?);
    println!("2.0");
    let u = Matrix::new(
        vec![2., 4., -2., -5., 3., 3., 0., 7., 4.],
        3,
        3,
    )?;
    println!("{}", u.trace()?);
    println!("9.0");
    let u = Matrix::new(
        vec![-2., 1., 0., -8., -23., 6., 4., 4., 4.],
        3,
        3,
    )?;
    println!("{}", u.trace()?);
    println!("-21.0");
    Ok(())
}
// Focuses on complex matrices, testing their transpose and adjunct operations
//to verify the mathematical properties of these transformations.
pub fn ex09_test() -> Result<(), LinAlgError> {
    let m1 = Matrix::<ComplexNumber>::new(
        vec![
            ComplexNumber::n(1.0, 0.),
            ComplexNumber::n(0., 1.0),
            ComplexNumber::n(-1., 1.),
            ComplexNumber::n(2., -1.),
            ComplexNumber::n(-2., 2.),
            ComplexNumber::n(-2., 0.),
        ],
        3,
        2,
    )?;
    println!("A complex matrix {}", m1);
    println!("Its transpose matrix {}", m1.tr()?);
    println!("Its adjunct matrix {}", m1.adj()?);
    Ok(())
}
// Examines the row echelon form conversion of matrices, checking for correct
// simplification and row reduction.
pub fn ex10_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(
        vec![1., 0., 0., 0., 1., 0., 0., 0., 1.],
        3,
        3,
    )?;
    println!("{}", u.row_echelon()?);
    println!(
        " 	// [1.0, 0.0, 0.0]
            // [0.0, 1.0, 0.0]
            // [0.0, 0.0, 1.0]"
    );
    let u = Matrix::new(vec![1., 2., 3., 4.], 2, 2)?;
    println!("{}", u.row_echelon()?);
    println!(
        "	// [1.0, 0.0]
            // [0.0, 1.0]"
    );
    let u = Matrix::new(vec![1., 2., 2., 4.], 2, 2)?;
    println!("{}", u.row_echelon()?);
    println!(
        "	// [1.0, 2.0]
            // [0.0, 0.0]"
    );
    let u = Matrix::new(
        vec![
            8., 4., 8., 5., 2.5, 5., -2., 20., 1., 4., 4.,
            4., 28., -4., 17.,
        ],
        3,
        5,
    )?;
    println!("{}", u.row_echelon()?);
    println!(
        "	// [1.0, 0.625, 0.0, 0.0, -12.1666667]
                // [0.0, 0.0, 1.0, 0.0, -3.6666667]
                // [0.0, 0.0, 0.0, 1.0, 29.5 ]"
    );
    Ok(())
}
// Tests the determinant calculation of matrices, ensuring accurate computation for matrices of various sizes and contents.
pub fn ex11_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(vec![1., -1., -1., 1.], 2, 2)?;
    println!("{}", u.determinant()?);
    println!("// 0.0");
    let u = Matrix::new(
        vec![2., 0., 0., 0., 2., 0., 0., 0., 2.],
        3,
        3,
    )?;
    println!("{}", u.determinant()?);
    println!("// 8.0");
    let u = Matrix::new(
        vec![8., 5., -2., 4., 7., 20., 7., 6., 1.],
        3,
        3,
    )?;
    println!("{}", u.determinant()?);
    println!("// -174.0");
    let u = Matrix::new(
        vec![
            8., 5., -2., 4., 4., 2.5, 20., 4., 8., 5., 1.,
            4., 28., -4., 17., 1.,
        ],
        4,
        4,
    )?;
    println!("{}", u.determinant()?);
    println!("// 1032");
    // 	XVI.1
    Ok(())
}

pub fn ex12_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(
        vec![1., 0., 0., 0., 1., 0., 0., 0., 1.],
        3,
        3,
    )?;
    println!("{}", u.inverse()?);
    println!(
        "	// [1.0, 0.0, 0.0]
                    // [0.0, 1.0, 0.0]
                    // [0.0, 0.0, 1.0]"
    );
    let u = Matrix::new(
        vec![2., 0., 0., 0., 2., 0., 0., 0., 2.],
        3,
        3,
    )?;
    println!("{}", u.inverse()?);
    println!(
        "// [0.5, 0.0, 0.0]
                    // [0.0, 0.5, 0.0]
                    // [0.0, 0.0, 0.5]"
    );
    let u = Matrix::new(
        vec![8., 5., -2., 4., 7., 20., 7., 6., 1.],
        3,
        3,
    )?;
    println!("{}", u.inverse()?);
    println!(
        "// [0.649425287, 0.097701149, -0.655172414]
                    // [-0.781609195, -0.126436782, 0.965517241]
                    // [0.143678161, 0.074712644, -0.206896552]"
    );
    Ok(())
}

pub fn ex13_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(
        vec![1., 0., 0., 0., 1., 0., 0., 0., 1.],
        3,
        3,
    )?;
    println!("{}", u.rank()?);
    println!("// 3");
    let u = Matrix::new(
        vec![
            1., 2., -1., 2., 4., 2., 0., 0., 1., 0., 0., 1.,
        ],
        3,
        4,
    )?;
    println!("{}", u.rank()?);
    println!("// 2");
    let u = Matrix::new(
        vec![
            8., 4., 7., 21., 5., 7., 6., 18., -2., 20., 1.,
            7.,
        ],
        4,
        3,
    )?;
    println!("{}", u.rank()?);
    println!("// 3");
    Ok(())
}

pub fn ex14_test() -> Result<(), LinAlgError> {
    println!(
        "{}",
        Matrix::<f64>::projection(
            80. * 3.14 / 180.,
            1.777,
            0.1,
            1000.
        )?
    );
    Ok(())
}

pub fn ex15_test() -> Result<(), LinAlgError> {
    //Complex number test
    println!("Complex number printing test");
    let x = ComplexNumber::Cartesian { re: 1.0, im: -1.0 };
    println!("the number {}", x);
    println!(
        "Norm: {} Multiplied by inv {}",
        x.clone() * x.conj(),
        x.clone() * x.inv()
    );
    println!("Its real part is {}", x.re());
    println!("Its complex part is {}", x.im());

    println!("Complex number printing test");
    let m1 = Matrix::<ComplexNumber>::new(
        vec![
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (1.0),
            },
            ComplexNumber::Cartesian {
                re: (-1.0),
                im: (1.0),
            },
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (1.0),
            },
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (1.0),
            },
        ],
        2,
        2,
    )?;
    println!("m1 = {}", m1);
    let m2 = Matrix::<ComplexNumber>::new(
        vec![
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (1.0),
            },
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (-1.0),
            },
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (1.0),
            },
            ComplexNumber::Cartesian {
                re: (1.0),
                im: (1.0),
            },
        ],
        2,
        2,
    )?;
    println!("m2 = {}", m2);
    let m3 = m1.mlt(&m2)?;
    println!("m3 = {}", m3);
    println!("compare the transpose and the adjunct");
    println!(
        "m1 transpose: {} m1 adjunct: {}",
        m1.tr()?,
        m1.adj()?
    );
    let m4 = Matrix::<ComplexNumber>::new(
        vec![
            ComplexNumber::n(3.0, 4.),
            ComplexNumber::n(0.32, 3.0),
            ComplexNumber::n(-1.4, 14.),
            ComplexNumber::n(24., -12.),
            ComplexNumber::n(-2., 2.),
            ComplexNumber::n(-32.3, 0.3),
            ComplexNumber::n(-2., -12.),
            ComplexNumber::n(-2., 2.),
            ComplexNumber::n(-2.3, -0.3),
        ],
        3,
        3,
    )?;
    println!("m4: {}", m4);
    println!("m4 determinant: {}", m4.determinant()?);
    println!("m4 inverse: {}", m4.inverse()?);
    println!(
        "m4 product with inverse: {}",
        m4.mul_mat(&m4.inverse()?)?
    );

    Ok(())
}

pub fn ex16_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(
        vec![1., 1., 1., 1., 1., 1., 1., 1., 1.],
        3,
        3,
    )?;
    println!("{}", u.inverse()?);
    Ok(())
}

pub fn basic_stats_test() -> Result<(), LinAlgError> {
    let u = Matrix::new(
        vec![
            8., 4., 7., 21., 5., 7., 6., 18., -2., 20., 1.,
            7.,
        ],
        4,
        3,
    )?;

    let v = u.column_extract(1)?;
    let (v2, mean_v, std_v) = v.normalize_vec()?;
    let (u2, coefs) = u.normalize_cols()?;
    println!(
        "Normalizing {} is {} with normalization coefs {}",
        u, u2, coefs
    );
    println!(
        "Extract the first column {v} and normalizing it {v2} we get mean\
     {mean_v} and standard deviation {std_v}"
    );
    println!(
        "Also 42 42 42 for the lolz {}",
        Matrix::<f64>::ones(42, 42)?.scl(42.)?
    );

    Ok(())
}
