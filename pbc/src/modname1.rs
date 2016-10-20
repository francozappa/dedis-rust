// src/modname1.rs
// vim: foldmethod=marker

// module doc {{{1
//! `modname1` is a mock module.
//!
//! Use modules as compilation units (incremental compilation), and as
//! namespaces.

// const {{{1
// NOTE: module scope
const UPPERCASE_NAME: &'static str = "This is a `str`";

// private {{{1
// NOTE: private declaration by-default

// not documented {{{2
fn private_fun() {
    println!("`private_fun` was called.");

    println!("{}", UPPERCASE_NAME);
}

// documented {{{2

// public {{{1

// not documented {{{2
pub fn cool() {
    println!("modname1 is public");
}

// documented {{{2
// NOTE: type and fn signatures already explain a lot

/// Call a private function through a public one.
pub fn call_me() {
    private_fun();
}


/// Always returns `1`.
pub fn unit1() -> u32 {
    1
}

/// Always returns `2`.
pub fn cratedoctest() -> u32 {
    2
}

// NOTE: Triple-slashes documents what is *after* them.
// NOTE: `cargo doc --open` and then just `cargo doc`
// NOTE: functionality summary in the first line.
/// Always returns `3`.
///
// NOTE: paragraph with more explanation
/// More information about how to document things may be found at:
/// https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md
///
// NOTE: special sections starts with `#`
// NOTE: conventional names: Panics, Errors, Safety, Examples
// NOTE: Panics: document when `fn` does `panic!`
/// # Panics
///
/// `fndoctest` does not `panic!`.
///
// NOTE: Errors: document a `Result<T, E>` returned value
/// # Errors
///
/// Describe under which condition the fn or the method returns `Err(E)`.
///
// NOTE: Safety: document `unsafe` code sections
/// # Safety
///
/// Explain which invariants the caller is responsible for upholding.
///
// NOTE: Examples: put testable examples
/// # Examples
///
/// This is a *basic* example:
///
// NOTE: code fence defaults to Rust, better be explicit tough
// NOTE: no need to add a `main` to test docs
/// ```rust
/// use pbc::modname1::fndoctest;
///
/// // NOTE: doctests supported only for lib
/// assert_eq!(3_u32, fndoctest());
/// ```
///
/// This example shows different code fences:
///
/// Code fence in c:
///
/// ```c
/// #include <stdio.h>
///
/// int main(void) {
///     printf("Hello c code fence.");
///
///     return 0;
/// }
/// ```
///
/// Code fence in plaintext:
///
/// ```text
/// this is plain-text
/// ```
///
/// Code fence in bash:
///
/// ```bash
/// cd ~; ls -la
/// ```
///
/// # Examples
///
/// This is an *advanced* example:
///
/// ```rust
/// use pbc::modname1::fndoctest;
///
/// // NOTE: doctests supported only for lib
/// // NOTE: fn doctests named: `test fndoctest_0` `test fndoc_1`
/// assert_eq!(3_u32, fndoctest());
/// ```
pub fn fndoctest() -> u32 {
    3
}

/// Show how to hide statement in the doc.
///
// FIXME: paragraph can be improved
/// This is useful when you want to present a `fn` doc like
/// a storyline. You can still test while you are explaining
/// but you have to tell the compiler to hide some statement
/// in the doc.
///
/// First, we set `x` to one:
///
/// ```rust
/// let x = 1;
/// # let y = 2;
/// # let y = 2;
/// # println!("{}", x + y);
/// ```
///
/// Next, we set `y` to two:
///
/// ```rust
/// # let x = 1;
/// let y = 2;
/// # let y = 2;
/// # println!("{}", x + y);
/// ```
///
/// Finally, we print the sum of `x` and `y`:
///
/// ```rust
/// # let x = 1;
/// # let y = 2;
/// # let y = 2;
/// println!("{}", x + y);
/// ```
///
pub fn fndoctest2() {
    let x = 1;
    let y = 2;

    println!("{}", x + y);
}

/// Shows `rustdoc` annotations.
///
/// You can use a comma `,` to list code fence annotation.
///
/// `rust,ignore` will not run the doctest:
///
/// ```rust,ignore
/// // ignored code
/// ```
/// See `fndoctest3_0` `ignored` in the test report
///
/// `rust,should_panic` works as the corresponding attribute:
///
/// ```rust,should_panic
/// assert!(false);
/// ```
/// See `fndoctest3_1` `ok` in the test report
///
/// `rust,no_run` to make sure it compiles without running it:
///
/// ```rust,no_run
/// loop {
///     println!("Infinite loop.");
/// }
/// ```
/// See `fndoctest3_2` `ok` in the test report
///
pub fn fndoctest3() {}


/// Images URLs can be included in the doc.
///
/// ![Alt version](https://www.rust-lang.org/logos/rust-logo-blk.svg)
///
pub fn fndoctest4() {}

// unit tests module {{{1

// FIXME: find a way to remove `#[test]` from all test function
// NOTE: `cargo test` will compile this module
// NOTE: use `cargo test --verbose` for more info
// NOTE: conditional compilation only with `cargo test`
#[cfg(test)]
mod tests {

    // NOTE: put parent module `pub` stuff in scope
    use super::*;

    // NOTE: `#[test]` functions must have `fn () -> ()` signature
    // #[test]
    // fn invalid_fn() -> i32 {
    //     3
    // }

    // NOTE: `cargo test` will execute this function
    #[test]
    fn it_passes() {
        // NOTE: passes because it does not `panic!`
    }

    #[test]
    #[ignore]
    fn it_fails() {
        // NOTE: `assert!` evaluates a boolean expression
        // NOTE: fails because it `panic!`
        assert!(false);
    }

    #[test]
    // NOTE: should_panic is fragile, use it carefully
    #[should_panic(expected = "assertion failed")]
    fn it_passes2() {
        // NOTE: `assert_eq!` compares two value of the same type
        // NOTE: passes because it `panic!`
        assert_eq!("Hello", "World");
    }

    #[test]
    #[ignore]
    // NOTE: `#[ignore]` to skip the unit test
    // NOTE: `cargo test -- --ignored` runs only the ignored
    // NOTE: `--ignored` in an argument to the test binary (not cargo)
    fn long_runtime() {
        assert_eq!("super long", "super long");
    }

    #[test]
    /// Tests a `pub fn` from the lib.
    fn parent_put_scope() {
        // NOTE: can use `unit1` because of `use super::*` above
        assert_eq!(1_u32, unit1())
    }

    #[test]
    fn private_fun_call() {
        // NOTE: can use `unit1` because of `use super::*` above
        call_me();
    }
}

// integration tests {{{1
// NOTE: add them if module is `pub`

/// Always returns `true`.
pub fn integration1() -> bool {
    println!("integration1 is `pub`");

    true
}
