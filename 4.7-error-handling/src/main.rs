use Event::NewRelease;
use std::fs::File;
use std::io;
use std::io::prelude::*;

enum Event {
    NewRelease,
}

struct Info {
    name: String,
    age: i32,
    rating: i32,
}

/*
  Result<T, E> Example

This is the signature of the Result object. We don't have
to define it since it comes with rust.
enum Result<T, E> {
    Ok(T), // This variant represents a success.
    Err(E) // This variant, an error.
}
 */

// The Debug trait is what lets us print the enum value using the {:?} format operation.
#[derive(Debug)]
enum Version { Version1, Version2 }

// We use an enum to enumerate all possible ParseErrors
#[derive(Debug)]
enum ParseError { InvalidHeaderLength, InvalidVersion }

fn parse_version(header: &u8) -> Result<Version, ParseError> {
    match *header {
        1 => Ok(Version::Version1),
        2 => Ok(Version::Version2),
        3 => Err(ParseError::InvalidHeaderLength), // not really, but still
        _ => Err(ParseError::InvalidVersion),

    }
}

fn test_version(header: &u8) {
    let version = parse_version(header);
    match version {
        Ok(v) => {
            println!("working with version: {:?}", v);
        }
        Err(e) => {
            println!("error parsing header: {:?}", e);
        }
    }
}

/*
 * `unreachable!()` Example
 */

fn probability(_: &Event) -> f64 {
    // real implementation would be more complex, of course
    0.95
}

fn descriptive_probability(event: Event) -> &'static str {
    match probability(&event) {
        1.00 => "certain",
        0.00 => "impossible",
        0.00 ... 0.25 => "very unlikely",
        0.25 ... 0.50 => "unlikely",
        0.50 ... 0.75 => "likely",
        0.75 ... 1.00 => "very likely",
        // Drop an `unreachable!()` macro here -- this should never occur!
        // if it does, it'll throw a panic
        _ => unreachable!(),
    }
}

/*
 * `try!()` Example
 */
fn write_info(info: &Info) -> io::Result<()> {
    // `unwrap()` panics if Err is returned.
    let mut file = File::create("file.txt").unwrap();

    // `try!` will result in either unwrapped success value `Ok`, or
    // if it errors, then returns from the function early, returning the `Err`
    try!(writeln!(&mut file, "name: {}", info.name));
    try!(writeln!(&mut file, "age: {}", info.age));
    try!(writeln!(&mut file, "rating: {}", info.rating));

    return Ok(());
}

fn main() {
    println!("{}", descriptive_probability(NewRelease));
    let versions = &[1, 2, 3];
    for version in versions {
        test_version(version);
    }

    let info = Info { name: "J".to_string(), age: 30, rating: 100 };
    match write_info(&info) {
        Ok(_) => println!("Stored!"),
        Err(err) => println!("Something went terribly wrong. {}", err),
    }
}
