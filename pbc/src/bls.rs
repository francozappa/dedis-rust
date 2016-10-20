// module doc {{{1
//! `modname2` is super cool.
//!
//! Use modules as compilation units (incremental compilation), and as
//! namespaces.

// const {{{1

// private {{{1

// not documented {{{2
#[allow(dead_code)]
fn cool() {
    println!("modname2 is not public");
}

// documented {{{2

// public {{{1

// not documented {{{2

// documented {{{2

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
