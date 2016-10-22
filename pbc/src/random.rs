// src/ecc.rs
// vim: foldmethod=marker

// doc {{{1
//! `random` contains object that interact with the `rand` crate
//!
//! https://doc.rust-lang.org/rand/rand/index.html#cryptographic-security
//!

// crate {{{1
extern crate rand;

// use {{{1

// `std` {{{2
use std::io;

// extern {{{2
use self::rand::OsRng;
use self::rand::Rng;

// internal {{{2

// public {{{1
// documented {{{2

/// Contains randomness used for secrets.
pub struct Stream {
    s: u32,
}

impl Stream {

    // FIXME: who decide the random integer range?
    /// Generate a fresh random number from 1 to 12345
    pub fn new() -> Stream {

        let rng = OsRng::new();
        let from: u32 = 1;
        let to: u32   = 12345;

        // FIXME: use combinator
        match rng {
            Ok(mut r)  => Stream {s: r.gen_range(from, to) },
            Err(e)     => panic!("Error: {}", e),
        }
    }

}

// not documented {{{2

// unit tests module {{{1
#[cfg(test)]
mod tests {

    use super::Stream;

    #[test]
    fn stream_range_ok() {
        let stream = Stream::new();

        assert!(stream.s >= 1 && stream.s <= 12345);
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
