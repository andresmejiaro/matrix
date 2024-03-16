use crate::traits::{Conj, Norm, One, Tf64};

// Define an enumeration `ComplexNumber` representing complex numbers in Cartesian form or representing infinity
#[derive(PartialEq, Clone, Copy)]
pub enum ComplexNumber {
    Cartesian { re: f64, im: f64 },
    Infinity,
}
// Implement the `One` trait for `ComplexNumber` to provide a method for creating a complex number with a value of 1
impl One for ComplexNumber {
    fn one() -> ComplexNumber {
        ComplexNumber::n(1.0, 0.0)
    }
}
// Implement the `Conj` trait for `ComplexNumber` to provide a method for computing the complex conjugate
impl Conj for ComplexNumber {
    fn conj(&self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian { re, im } => {
                ComplexNumber::Cartesian {
                    re: *re,
                    im: -*im,
                }
            }
            ComplexNumber::Infinity => {
                ComplexNumber::Infinity
            }
        }
    }
}
// Implement the `Tf64` trait for `ComplexNumber` to convert complex numbers to an `f64` representation based on their real part
impl Tf64 for ComplexNumber {
    fn tf64(&self) -> f64 {
        self.re()
    }
}
// Implement the `Norm` trait for `ComplexNumber` to compute the Euclidean norm of the complex number
impl Norm for ComplexNumber {
    fn norm(&self) -> f64 {
        match self {
            ComplexNumber::Infinity => f64::NAN,
            ComplexNumber::Cartesian { re, im } => {
                (re * re + im * im).powf(0.5)
            }
        }
    }
}
// Implement the `Default` trait for `ComplexNumber` to provide a default value
impl Default for ComplexNumber {
    fn default() -> Self {
        ComplexNumber::Cartesian { re: 0.0, im: 0.0 }
    }
}
// Provide methods for accessing the real and imaginary parts, creating a new complex number, and computing the inverse
impl ComplexNumber {
    pub fn re(&self) -> f64 {
        match self {
            ComplexNumber::Cartesian { re, im: _ } => *re,
            ComplexNumber::Infinity => f64::NAN,
        }
    }

    pub fn im(&self) -> f64 {
        match self {
            ComplexNumber::Cartesian { re: _, im } => *im,
            ComplexNumber::Infinity => f64::NAN,
        }
    }
    pub fn n(re: f64, im: f64) -> ComplexNumber {
        ComplexNumber::Cartesian { re: re, im: im }
    }

    pub fn inv(&self) -> ComplexNumber {
        match self {
            ComplexNumber::Infinity => {
                ComplexNumber::Cartesian {
                    re: 0.0,
                    im: 0.0,
                }
            }
            ComplexNumber::Cartesian { re, im } => {
                if *re == 0.0 && *im == 0.0 {
                    ComplexNumber::Infinity
                } else {
                    ComplexNumber::Cartesian {
                        re: *re
                            / (*re * (*re) + (*im) * (*im)),
                        im: -*im
                            / ((*re) * (*re)
                                + (*im) * (*im)),
                    }
                }
            }
        }
    }
}

use std::ops::Add;
// Implement the `Add` trait for `ComplexNumber` to provide addition functionality
impl Add for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, other: Self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian {
                re: re1,
                im: im1,
            } => match other {
                ComplexNumber::Cartesian {
                    re: re2,
                    im: im2,
                } => ComplexNumber::Cartesian {
                    re: re1 + re2,
                    im: im1 + im2,
                },
                ComplexNumber::Infinity => {
                    ComplexNumber::Infinity
                }
            },
            ComplexNumber::Infinity => {
                ComplexNumber::Infinity
            }
        }
    }
}

use std::ops::Sub;
// Implement the `Sub` trait for `ComplexNumber` to provide subtraction functionality
impl Sub for ComplexNumber {
    type Output = ComplexNumber;
    fn sub(self, other: Self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian {
                re: re1,
                im: im1,
            } => match other {
                ComplexNumber::Cartesian {
                    re: re2,
                    im: im2,
                } => ComplexNumber::Cartesian {
                    re: re1 - re2,
                    im: im1 - im2,
                },
                ComplexNumber::Infinity => {
                    ComplexNumber::Infinity
                }
            },
            ComplexNumber::Infinity => {
                ComplexNumber::Infinity
            }
        }
    }
}

use std::ops::Mul;
// Implement the `Mul` trait for `ComplexNumber` to provide multiplication functionality
impl Mul for ComplexNumber {
    type Output = ComplexNumber;
    fn mul(self, other: Self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian {
                re: re1,
                im: im1,
            } => match other {
                ComplexNumber::Cartesian {
                    re: re2,
                    im: im2,
                } => ComplexNumber::Cartesian {
                    re: re1 * re2 - im1 * im2,
                    im: re2 * im1 + re1 * im2,
                },
                ComplexNumber::Infinity => {
                    ComplexNumber::Infinity
                }
            },
            ComplexNumber::Infinity => {
                ComplexNumber::Infinity
            }
        }
    }
}

use std::ops::Div;
// Implement the `Div` trait for `ComplexNumber` to provide division functionality
impl Div for ComplexNumber {
    type Output = ComplexNumber;
    fn div(self, other: Self) -> ComplexNumber {
        self * other.inv()
    }
}

use std::fmt;
// Implement the `fmt::Display` trait for `ComplexNumber` to enable custom formatting when printed
impl fmt::Display for ComplexNumber {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            ComplexNumber::Infinity => write!(f, "Inf"),
            ComplexNumber::Cartesian { re, im } => {
                write!(f, "{re} + {im}i")
            }
        }
    }
}

use std::ops::Neg;
// Implement the `Neg` trait for `ComplexNumber` to provide negation functionality
impl Neg for ComplexNumber {
    type Output = ComplexNumber;
    fn neg(self) -> Self::Output {
        match self {
            ComplexNumber::Cartesian { re, im } => {
                ComplexNumber::Cartesian {
                    re: -re,
                    im: -im,
                }
            }
            ComplexNumber::Infinity => {
                ComplexNumber::Infinity
            }
        }
    }
}
