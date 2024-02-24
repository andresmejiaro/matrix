// One

pub trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> f32 {
        1.0
    }
}

impl One for f64 {
    fn one() -> f64 {
        1.0
    }
}

// Tf64
pub trait Tf64 {
    fn tf64(&self) -> f64;
}

impl Tf64 for f64 {
    fn tf64(&self) -> f64 {
        *self
    }
}

impl Tf64 for f32 {
    fn tf64(&self) -> f64 {
        *self as f64
    }
}

// norm

pub trait Norm {
    fn norm(&self) -> f64;
}

impl Norm for f64 {
    fn norm(&self) -> f64 {
        if *self < 0.0 {
            return -*self;
        }
        *self
    }
}

impl Norm for f32 {
    fn norm(&self) -> f64 {
        if *self < 0.0 {
            return -*self as f64;
        }
        *self as f64
    }
}

// conj

pub trait Conj {
    fn conj(&self) -> Self;
}

impl Conj for f64 {
    fn conj(&self) -> f64 {
        *self
    }
}

impl Conj for f32 {
    fn conj(&self) -> f32 {
        *self
    }
}
