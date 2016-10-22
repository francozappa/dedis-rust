// src/ecc.rs
// vim: foldmethod=marker

// doc {{{1
//! `ecc` contains Elliptic Curve Crypto objects.
//!

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
    s: u32,
}

impl Scalar {
    pub fn new(s: u32) -> Scalar { Scalar { s: s } }
    pub fn one() -> Scalar { Scalar { s: 1 } }

    /// Set method
    pub fn pick<S>(&mut self, s: u32) { self.s = s }
}

/// Two dimensional point.
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point { Point { x: x, y: y } }
    pub fn origin() -> Point { Point { x: 0.0, y: 0.0 } }
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
