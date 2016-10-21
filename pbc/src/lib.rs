// src/lib.rs
// vim: foldmethod=marker

// TODO: ship to crates.io, (spell check)
// TODO: code on GitHub (with badges)
// TODO: news on user forum and reddit

// NOTE: doctests supported only for lib
// crate doc {{{1
//! `pbc` is crate implementing a pairing-based cryptosystem
//!
//! # Goals
//!
//! * TODO
//! * Instructive to learn (together) how to strucute a crate
//!
//! # Usage
//!
//! TODO
//!
//! # Examples
//!
//! ```
//! use pbc::*;
//!
//! assert!(true);
//! ```
//!

// config {{{1
#![crate_type= "lib"]

// imports {{{1
// extern crate {{{2

// mod {{{1
// NOTE: `mod` exports only `pub` objects

// public {{{2
pub mod ecc;
pub mod bls;
pub mod random;

// private {{{2

// use {{{1

// `std` {{{2
#[allow(unused_imports)]
use std::sync::{Arc, RwLock};

// extern {{{2

// internal {{{2
// use ecc::{Point, Scalar};


// examples {{{1
// NOTE: show APIs and use cases
// NOTE: see examples folder

