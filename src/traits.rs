// Define a trait `One` for types that can represent the concept of one
pub trait One {
    fn one() -> Self;
}
// Implement the `One` trait for `f32` returning 1.0
impl One for f32 {
    fn one() -> f32 {
        1.0
    }
}
// Implement the `One` trait for `f64` returning 1.0
impl One for f64 {
    fn one() -> f64 {
        1.0
    }
}

// Define a trait `Tf64` for converting a value to `f64`
pub trait Tf64 {
    fn tf64(&self) -> f64;
}
// Implement the `Tf64` trait for `f64` to return itself
impl Tf64 for f64 {
    fn tf64(&self) -> f64 {
        *self
    }
}
// Implement the `Tf64` trait for `f32` to convert to `f64`
impl Tf64 for f32 {
    fn tf64(&self) -> f64 {
        *self as f64
    }
}

// Define a trait `Norm` for computing the norm of a value
pub trait Norm {
    fn norm(&self) -> f64;
}
// Implement the `Norm` trait for `f64` to return its absolute value
impl Norm for f64 {
    fn norm(&self) -> f64 {
        if *self < 0.0 {
            return -*self;
        }
        *self
    }
}
// Implement the `Norm` trait for `f32` to return its absolute value as `f64`
impl Norm for f32 {
    fn norm(&self) -> f64 {
        if *self < 0.0 {
            return -*self as f64;
        }
        *self as f64
    }
}

// conj
// Define a trait `Conj` for computing the conjugate of a value
pub trait Conj {
    fn conj(&self) -> Self;
}
// Implement the `Conj` trait for `f64` to return itself
impl Conj for f64 {
    fn conj(&self) -> f64 {
        *self
    }
}
// Implement the `Conj` trait for `f32` to return itself
impl Conj for f32 {
    fn conj(&self) -> f32 {
        *self
    }
}

use crate::complex::ComplexNumber;
use ::core::ops::{Add, Div, Mul, Neg, Sub};

// Define a trait `Field` that encompasses operations and traits for field elements
pub trait Field:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + Copy
    + Default
    + One
    + Tf64
    + Norm
    + Conj
    + Neg
    + std::fmt::Display
{
}
// Implement the `Field` trait for `f64` 'f32' and ComplexNumber
impl Field for f64 {}
impl Field for f32 {}
impl Field for ComplexNumber {}
