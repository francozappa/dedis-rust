// src/ecc.rs
// vim: foldmethod=marker

// doc {{{1
//! `random` contains object that interact with the `rand` crate
//!

// public {{{1
// documented {{{2
// not documented {{{2

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
