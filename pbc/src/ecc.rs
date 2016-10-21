// src/ecc.rs
// vim: foldmethod=marker

// doc {{{1
//! `ecc` contains Elliptic Curve Crypto objects.
//!

// public {{{1

// documented {{{2
/// Group interface.
pub trait Group {
    fn scalar_len(&self) -> u32;       // Max len of scalars in bytes
    fn scalar(&self) -> Scalar;        // Create a new scalar
    fn point_len(&self) -> u32;        // Max len of point in bytes
    fn point(&self) -> Point;          // Create a new point
    fn prime_order(&self) -> bool;     // Returns `true` if group is prime-order
}

/// Positive integer scalar.
pub struct Scalar {
    n: u32,
}

/// Two dimensional point.
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {x: x, y: y }
    }
}


// not documented {{{2

// unit tests module {{{1
#[cfg(test)]
mod tests {

    #[test]
    fn it_passes() {
        assert!(true);
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
