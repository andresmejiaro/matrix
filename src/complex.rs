use crate::traits::{Conj, Norm, One, Tf64};

#[derive(PartialEq, Clone, Copy)]
pub enum ComplexNumber {
    Cartesian { re: f64, im: f64 },
    Infinity,
}

impl One for ComplexNumber {
    fn one() -> ComplexNumber {
        ComplexNumber::n(1.0, 0.0)
    }
}

impl Conj for ComplexNumber {
    fn conj(&self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian { re, im } => {
                ComplexNumber::Cartesian { re: *re, im: -*im }
            }
            ComplexNumber::Infinity => ComplexNumber::Infinity,
        }
    }
}

impl Tf64 for ComplexNumber {
    fn tf64(&self) -> f64 {
        self.re()
    }
}

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

impl Default for ComplexNumber {
    fn default() -> Self {
        ComplexNumber::Cartesian { re: 0.0, im: 0.0 }
    }
}

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
                ComplexNumber::Cartesian { re: 0.0, im: 0.0 }
            }
            ComplexNumber::Cartesian { re, im } => {
                if *re == 0.0 && *im == 0.0 {
                    ComplexNumber::Infinity
                } else {
                    ComplexNumber::Cartesian {
                        re: *re / (*re * (*re) + (*im) * (*im)),
                        im: -*im / ((*re) * (*re) + (*im) * (*im)),
                    }
                }
            }
        }
    }
}

use std::ops::Add;

impl Add for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, other: Self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian { re: re1, im: im1 } => match other {
                ComplexNumber::Cartesian { re: re2, im: im2 } => {
                    ComplexNumber::Cartesian {
                        re: re1 + re2,
                        im: im1 + im2,
                    }
                }
                ComplexNumber::Infinity => ComplexNumber::Infinity,
            },
            ComplexNumber::Infinity => ComplexNumber::Infinity,
        }
    }
}

use std::ops::Sub;

impl Sub for ComplexNumber {
    type Output = ComplexNumber;
    fn sub(self, other: Self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian { re: re1, im: im1 } => match other {
                ComplexNumber::Cartesian { re: re2, im: im2 } => {
                    ComplexNumber::Cartesian {
                        re: re1 - re2,
                        im: im1 - im2,
                    }
                }
                ComplexNumber::Infinity => ComplexNumber::Infinity,
            },
            ComplexNumber::Infinity => ComplexNumber::Infinity,
        }
    }
}

use std::ops::Mul;

impl Mul for ComplexNumber {
    type Output = ComplexNumber;
    fn mul(self, other: Self) -> ComplexNumber {
        match self {
            ComplexNumber::Cartesian { re: re1, im: im1 } => match other {
                ComplexNumber::Cartesian { re: re2, im: im2 } => {
                    ComplexNumber::Cartesian {
                        re: re1 * re2 - im1 * im2,
                        im: re2 * im1 + re1 * im2,
                    }
                }
                ComplexNumber::Infinity => ComplexNumber::Infinity,
            },
            ComplexNumber::Infinity => ComplexNumber::Infinity,
        }
    }
}

use std::ops::Div;

impl Div for ComplexNumber {
    type Output = ComplexNumber;
    fn div(self, other: Self) -> ComplexNumber {
        self * other.inv()
    }
}

use std::fmt;

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplexNumber::Infinity => write!(f, "Inf"),
            ComplexNumber::Cartesian { re, im } => write!(f, "{re} + {im}i"),
        }
    }
}

use std::ops::Neg;

impl Neg for ComplexNumber {
    type Output = ComplexNumber;
    fn neg(self) -> Self::Output {
        match self {
            ComplexNumber::Cartesian { re, im } => {
                ComplexNumber::Cartesian { re: -re, im: -im }
            }
            ComplexNumber::Infinity => ComplexNumber::Infinity,
        }
    }
}
