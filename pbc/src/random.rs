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
pub struct Stream(pub u64);

impl Stream {
    /// Generate a fresh random number from 1 to 12345
    pub fn from_to(from: u64, to: u64) -> Stream {

        let rng = OsRng::new();

        // FIXME: use combinator
        match rng {
            Ok(mut r)  => Stream(r.gen_range(from, to)),
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
        let stream = Stream::from_to(1, 12345);

        assert!(stream.0 >= 1 && stream.0 <= 12345);
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
