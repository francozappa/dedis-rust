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

    /// Generate a fresh random number in a `from` `to` interval.
    fn new(from: u32, to: u32) -> io::Result<Stream> {

        let rng = OsRng::new();

        // FIXME: use combinator
        match rng {
            Ok(mut r)  => Ok(Stream {s: r.gen_range(from, to) }),
            Err(e) => Err(e),
        }
    }

}

// not documented {{{2

// unit tests module {{{1
#[cfg(test)]
mod tests {

    use super::Stream;

    #[test]
    fn rand_range() {
        let stream = Stream::new(1, 12345).unwrap();

        assert!(stream.s >= 1 && stream.s <= 12345);
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`
