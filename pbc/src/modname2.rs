// module doc {{{1
//! `modname2` is super cool.
//!
//! Use modules as compilation units (incremental compilation), and as
//! namespaces.

// const {{{1
// NOTE: module scope
const ROUNDS: u8 = 24;

// private {{{1
// NOTE: private declaration by-default

// not documented {{{2

// documented {{{2
/// Always returns the `ROUNDS` `const`.
pub fn unit2() -> u8 {
    ROUNDS
}

#[allow(dead_code)]
fn cool() {
    println!("modname2 is not public");
}

// public {{{1

// not documented {{{2

// documented {{{2

// unit tests module {{{1

// FIXME: find a way to remove `#[test]` from all test function
// NOTE: `cargo test` will compile this module
// NOTE: conditional compilation only with `cargo test`
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
