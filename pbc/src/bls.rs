// src/bls.rs
// vim: foldmethod=marker

// doc {{{1
//! `bls` implements Boneh, Lynn, and Shacam (BLS) short signature scheme.
//!

// use {{{1

// `std` {{{2

// extern {{{2

// internal {{{2
use ecc::{Point, Scalar, Group};
use random::{Stream};

// const {{{1

// private {{{1

// documented {{{2

// not documented {{{2

// public {{{1
// documented {{{2

// not documented {{{2
pub struct Pairing<'a> {
    name: &'a str,
    g1: Group<'a>,
    g2: Group<'a>,
    gt: Group<'a>,
}

pub struct BLS<'a> {
    name: &'a str,
    g: Point,
    // TODO
    // p: Pairing<'a>,
}

impl<'a> BLS<'a> {
    pub fn new() -> BLS<'a> {
        BLS {
            name: "Boneh-Lynn-Shacam short signatures scheme",
            // FIXME: set correct value
            g: Point::new(1.0, 2.0),
            // p : TODO
        }
    }

    /// Returns the public group generator.
    pub fn generator(&self) -> &Point { &self.g }

    /// Init and move a `Scalar`struct.
    pub fn scalar(&self)    -> Scalar { Scalar::one() }

    /// Init and move a `Point` struct.
    pub fn point(&self)     -> Point { Point::origin() }

    /// Returns the message `m` digest using MapToGroup.
    ///
    /// More info about the algorithm on the reference paper.
    pub fn hash(m: &str)    -> String { unimplemented!() }
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
