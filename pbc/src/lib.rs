// src/lib.rs

// TODO: ship to crates.io, (spell check)
// TODO: code on GitHub (with badges)
// TODO: news on user forum and reddit

// NOTE: `//!` documents the enclosing item
// NOTE: Eg: use it for module or crates documentation

// crate doc {{{1
//! `pbc` is crate implementing a pairing-based cryptosystem
//!
//! # Goals
//!
//! * Re-usable boilerplate code
//! * Instructive to learn (together) how to strucute a crate
//!
//! # Usage
//!
//! Download it, try it, and fix it.
//!
//! # Examples
//!
//! ```
//! use pbc::*;
//!
//! // NOTE: doctests supported only for lib
//! assert_eq!(2_u32, modname1::cratedoctest());
//! ```
//!
//! # Examples
//!
//! ```
//! use pbc::*;
//!
//! // NOTE: crate doctests named: `test _0` `test _1`
//! assert_eq!(2_u32, modname1::cratedoctest());
//! ```

// config {{{1
#![crate_type= "lib"]

// imports {{{1
// extern crate {{{2

// mod {{{1
// NOTE: `mod` exports only `pub` objects

// public {{{2
// NOTE: expects `modname1` into `src`
pub mod modname1;

// private {{{2
// NOTE: expects `modname2` into `src`
mod modname2;

// use {{{1
// NOTE: `use` enables to use short qualifiers

// `std` {{{2
// NOTE: `std` is always included in the main file

#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

// extern {{{2
#[allow(unused_imports)]

// internal {{{2
// TODO


// examples {{{1
// NOTE: show APIs and use cases

pub fn functionality1() {
    println!("functionality1 is `pub`");
}

