// src/bls.rs
// vim: foldmethod=marker

// doc {{{1
//! `bls` implements Boneh, Lynn, and Shacam (BLS) short signature scheme.
//!

// const {{{1

// private {{{1

// documented {{{2

// not documented {{{2
#[allow(dead_code)]
fn cool() {
    println!("modname2 is not public");
}

// public {{{1
// documented {{{2
/// Interface to the BLS struct.
#[inline]
#[allow(non_snake_case)]
pub fn BLS() -> BLS {
    BLS::new()
}

/// Positive integer scalar.
pub struct Scalar {
    x: u32,
}

/// Two dimensional point.
pub struct Point {
    x: f32,
    y: f32,
}
// not documented {{{2
#[allow(non_snake_case)]
pub struct BLS {
    pub GP: Point,
}

impl BLS {

    fn new() -> BLS {
        BLS {
            GP: Point { x: 0.0, y: 0.0 },
        }
    }

}


// unit tests module {{{1
#[cfg(test)]
mod tests {

    // NOTE: put parent module `pub` stuff in scope
    use super::*;

    // NOTE: `cargo test` will execute this function
    #[test]
    fn it_passes() {
        assert_eq!(24_u8, unit2());
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
