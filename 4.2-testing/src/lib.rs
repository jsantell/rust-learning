//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Group all of our tests together and define helper functions
// that don't become part of the rest of our crate. `cfg` means
// this only builds if we are running tests
#[cfg(test)]
// can expect a panic response
// #[should_panic]
//
// Can provide an expectation that the panic must have
// a matching string
// #[should_panic(expected = "assertion failed")]
mod tests {
    use super::add_two; // access the super scope to bring in the fn
    // use super::*;
    // could also just bring in everything

    #[test]
    fn test_add_two() {
        assert!(true);
        assert_eq!(4, add_two(2));
    }
}
