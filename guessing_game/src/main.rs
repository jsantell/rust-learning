// Like `use rand`, so can use `rand::<method>` anywhere
// and its in our Cargo.toml's dependencies
extern crate rand;

use std::io;
use rand::Rng;
// Bring `Ordering` into scope
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Because we used `use rand::Rng`, the `gen_range` method is available
    // TODO I don't quite get this.
    // `thread_rng()` requires `rand::Rng` be in scope, as methods are defined
    // on "traits" for the method to work, and the trait needs to be in scope
    // ???
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Loops indefinitely until `break`
    loop {
        println!("Please input your guess.");

        // `let mut guess`
        // Use `mut` to define this variable as mutable, as opposed
        // to the immutable default

        // `String::new()`
        // String is a growable, UTF-8 encoded bit of text
        // :: accesses the static method
        let mut guess = String::new();

        // Use the `stdin` function from `io` library.
        // If we didn't `use std::io`, we could access via `std::io::stdin`
        io::stdin() // Returns a handle to standard input, `std::io::Stdin`
            .read_line(&mut guess) // Pass in our handle to the method
                                   // We need to make this reference mutable as well, via `mut` here.
            .ok() // `.read_line(&mut String)` returns
                  // a value as well as populating the reference, of type `io::Result`.
                  // `io::Result` type has an `ok()` method, meaning assume this is successful
            .expect("Failed to read line"); // The `ok()` method returns a value with an `.expect()`
                                            // method, calling `panic!` with the message if the Result
                                            // failed.

        // Even though we already have a `guess` variable,
        // we can "shadow" the previous `guess` with this one, used often
        // in type casting scenarios.

        // `: u32` we are annotating `guess`'s type, with an unsigned, 32-bit integer,
        // needed so parse() knows what to return
        let guess: u32 = match guess.trim().parse() {
            // a match statement from `parse()`s Result enum, which has
            // Ok() and Err() variants, each called with additional information,
            // kind of like `(err, value) => {}` JavaScript callbacks?
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // `{}` substitutes the values in the string

        // Ordering is an enum, looks like:
        // enum Ordering {
        //   Less,
        //   Greater,
        //   Equal,
        // }
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // Break when you actually win
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
