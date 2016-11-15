// src/ecc.rs
// vim: foldmethod=marker

// doc {{{1
//! `ecc` contains Elliptic Curve Crypto objects.
//!

use random::Stream;

// public {{{1

// documented {{{2
/// Group interface.
pub struct Group<'a> {
    name: &'a str
}

impl<'a> Group<'a> {
    /// Max len of scalars in bytes
    fn scalar_len(&self) -> u32 { unimplemented!() }

    /// Create a new scalar
    fn scalar(&self) -> Scalar { unimplemented!(); }

    /// Max len of point in bytes
    fn point_len(&self) -> u32 { unimplemented!(); }

    /// Create a new point
    fn point(&self) -> Point   { unimplemented!(); }

    /// Returns `true` if group is prime-order
    fn prime_order(&self) -> bool { unimplemented!(); }
}

/// Positive integer scalar.
pub struct Scalar {
    s: u64,
}

impl Scalar {
    pub fn new(s: u64) -> Scalar { Scalar { s: s } }
    pub fn one()       -> Scalar { Scalar { s: 1_u64 } }
    pub fn random(from: u64, to: u64) -> Scalar {
        let r: u64 = Stream::from_to(from, to).0;

        Scalar { s: r }
    }
}

/// Two dimensional point.
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point { Point { x: x, y: y } }
    pub fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
}


// not documented {{{2

// unit tests module {{{1
#[cfg(test)]
mod tests {

    // #[test]
    fn it_passes() {
        assert!(true);
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
