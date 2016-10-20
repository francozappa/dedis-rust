// tests/integration1.rs

// NOTE: `cargo test` will compile each of tests file in a separate crate

// NOTE: link test crate to your library
extern crate pbc;

// NOTE: called functions must come from public modules
#[test]
fn wow() {
    assert!(pbc::modname1::integration1());
}

