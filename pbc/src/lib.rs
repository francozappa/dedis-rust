// src/lib.rs
// vim: foldmethod=marker

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
pub mod modname1;
pub mod bls;

// private {{{2

// use {{{1
// NOTE: `use` enables to use short qualifiers

// NOTE: `std` is always included in the main file
// `std` {{{2
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

// extern {{{2

// internal {{{2

// traits (aka interfaces) {{{1

// NOTE: ported from abstract/group.go
pub trait Group {
    StringId(&self) -> String;    // FIXME: used for?
    ScalarLen(&self) -> u32;      // Max len of scalars in bytes
    Scalar(&mut self) -> Scalar;  // Create a new scalar
    PointLen(&self) -> u32;       // Max len of point in bytes
    Point(&self) -> Point;        // Create a new point
    PrimeOrder(&self) -> bool     // Returns `true` if group is prime-order
}


// examples {{{1
// NOTE: show APIs and use cases

pub fn functionality1() {
    println!("functionality1 is `pub`");
}

