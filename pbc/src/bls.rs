// src/bls.rs
// vim: foldmethod=marker

// doc {{{1
//! `bls` implements Boneh, Lynn, and Shacam (BLS) short signature scheme.
//!

// use {{{1

// `std` {{{2

// extern {{{2

// internal {{{2
use ecc::{Point, Scalar};
use random::{Stream};

// const {{{1

// private {{{1

// documented {{{2

// not documented {{{2

// public {{{1
// documented {{{2

#[inline]
/// Interface to the BLS struct.
pub fn bls() -> BLS { BLS::new() }

// not documented {{{2
pub trait BLSScheme {
    type G;

    /// Returns the public group generator.
    fn generator(&self) -> &Point;

    /// Init and returns a `Scalar` struct.
    fn scalar(&self)    -> Scalar;

    /// Pick a scalar in the correct range.
    fn pick(&self)      -> Stream;

    /// Returns the message `m` digest using MapToGroup.
    ///
    /// More info about the algorithm on the reference paper.
    fn hash(m: &str)    -> String;
}

pub struct BLS {

    generator: Point,

}

impl BLS {
    fn new() -> BLS {

        BLS {
            // NOTE: generator is public
            // FIXME: set correct value
            generator: Point::new(1.0, 2.0),
        }
    }
}

impl BLSScheme for BLS {

    // FIXME: use the correct group
    type G = i32;

    fn generator(&self) -> &Point { &self.generator }
    fn scalar(&self)    -> Scalar { unimplemented!() }
    fn pick(&self)      -> Stream { unimplemented!() }
    fn hash(m: &str)    -> String { unimplemented!() }

}


// unit tests module {{{1
#[cfg(test)]
mod tests {

    use super::BLS;

    #[test]
    fn it_passes() {
        assert!(true);
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
