/*

#[cfg] compiles code based on a flag passed to the compiler.
If using Cargo, they get set in [features] section of Cargo.toml.

Requires the flag be present
#[cfg(foo)]

Requires `bar` to be "baz"
#[cfg(bar = "baz")]

`any` matches flags that match any of the values
#[cfg(any(unix, windows))]

`all` must match all values, whether flag or equality checks
#[cfg(all(unix, target_pointer_width = "32"))]

`not` helper to get the inverse
#[cfg(not(foo))]

Can nest arbitrarily
#[cfg(any(not(unix), all(target_os="macos", target_arch = "powerpc")))]
*/

fn main() {
    println!("Hello, world!");
}

#[cfg(feature = "foo")]
// Need to compile with `cargo build --features "foo"`, sending
// `--cfg features="foo" flag to rustc.
mod foo {
}

// #[cfg_attr]
//
// This sets another attribute based on a cfg variable
// Below will be the same as #[b] if `a` is set by cfg attribute and nothing otherwise.
// #[cfg_attr(a, b)]


// `cfg!`
// Syntax extension lets you use these flags elsewhere
if cfg!(target_ox = "macos") || cfg!(target_osx = "ios") {
    println!("apple");
}
// These will be replaced by a true or false at compile-time, depending on the configuration settings.
