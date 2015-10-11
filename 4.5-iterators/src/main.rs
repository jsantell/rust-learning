// Iterator docs for more info
// https://doc.rust-lang.org/stable/std/iter/

fn main() {
    // Here is a range (0..10) which is an iterator,
    // something we can call `.next()` on repeatedly and
    // returns a sequence of things
    for x in 0..10 {
        println!("{}", x);
    }

    // We can use loops and pattern matching too
    let mut range = 0..10;
    loop {
        // .next() returns a `Option<i32>`, which will be
        // Some<i32> when we have a value and None once we run out.
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }

    // /!\ ANTI PATTERN /!\
    // Dont do this!
    // You can iterate over vectors directly.
    // This is less semantic/clear what's happening, and requires bounds checking
    let nums = vec![1, 2, 3];
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    // INstead do this:
    for num in &nums {
        println!("{}", num);
        // `println!` is a macro function, automatically dereferencing
        // `num` for us, we could also just do
        // println!("{}", *num);
    }

    // There are three broad classes of relevent things here:
    //
    // *iterators* give you a sequence of values.
    // *iterator adapters* operate on an iterator, producing a new iterator with a different
    // output sequence.
    // *consumers* operate on an iterator, producing some final set of values.


    // A consumer operates on an iterator, returning some kind of value or values.
    // `collect()` is a common one:
    // let one_to_one_hundred = (1..101).collect();
    //
    // but that won't compile, because it doesn't know what type of things
    // rust should collect, so lets give it a type
    let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
    // `::<>` syntax allows us to give a type hint. Sometimes we can get away with
    // placeholder type, where rust figures out what `_` is
    let one_to_one_hundred = (1..101).collect::<Vec<_>>();

    // `find` is another common consumer, taking a closure and
    // works on a reference to each element.
    let greater_than_forty_two = (0..100)
                                 .find(|x| *x > 42);
    // `find` returns an Option because we may or may not find
    // the correct element.
    match greater_than_forty_two {
        Some(_) => println!("We got some numbers!"),
        None => println!("No numbers found :("),
    }

    // `fold` is another consumer -- it's like `Array.prototype.reduce` in
    // JS, with different function signature
    let sum = (1..4).fold(0, |sum, x| sum + x);

    // Iterators are lazy!
    let nums = 0..100; // This doesn't generate values 0 to 100 up front
                      // until we iterate over them, like `.collect()`

    // Other than ranges, we can also use `.iter()` on a vector for the same result
    let nums = vec![1, 2, 3];
    for num in nums.iter() {
        println!("{}", num);
    }

    // Iterator adapters
    //
    // Takes an iterator and returns another iterator, like `map`
    (1..100).map(|x| x + 1);

    // Filter
    for i in (1..10).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }

    // Since these are all iterator-producing methods, we can chain them together
    for i in (1..)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect::<Vec<i32>>() {
        println!("chainable: {}", i);
    }
}
