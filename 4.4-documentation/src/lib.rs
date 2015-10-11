//! A module for using squaring things.
//!
//! The `foo` module contains a lot of useful functionality blah blah blah
//! This is documentation for a module

#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://doc.rust-lang.org/")]

/// Squares `value` and returns the value.
///
/// # Examples
///
/// ```
/// let hundred = docs::square(10);
/// assert_eq!(hundred, 100);
/// ```
///
/// And some non-rust code:
/// ```js
/// _.map(x => x * x);
/// ```
///
/// # Panics
///
/// Unrecoverable misuses of a function (i.e. programming errors) in Rust are usually indicated by
/// panics, which kill the whole current thread at the very least. If your function has a
/// non-trivial contract like this, that is detected/enforced by panics, documenting it is very
/// important.
///
/// # Failures
///
/// If your function or method returns a Result<T, E>, then describing the conditions under which
/// it returns Err(E) is a nice thing to do. This is slightly less important than Panics, because
/// failure is encoded into the type system, but it's still a good thing to do.
///
/// # Safety
/// If your function is unsafe, you should explain which invariants the caller is responsible for
/// upholding.
pub fn square(value: usize) -> usize {
    value * value
}


/// Panic with a given message unless an expression evaluates to true.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate docs;
/// # fn main() {
/// panic_unless!(1 + 1 == 2, "Math is broken.");
/// # }
/// ```
///
/// ```should_panic
/// # #[macro_use] extern crate docs;
/// # fn main() {
/// panic_unless!(true == false, "I'm broken.");
/// # }
/// ```
#[macro_export]
macro_rules! panic_unless {
    ($condition:expr, $($rest:expr),+) => ({ if ! $condition { panic!($($rest),+); } });
}
