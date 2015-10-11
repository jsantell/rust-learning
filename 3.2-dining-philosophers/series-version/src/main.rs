use std::thread;

// Create a struct that represents a philosopher
// Store a property "name" of type `String` -- generally
// better for structures to own its own data, rather than using
// references
struct Philosopher {
    name: String,
}

// We define things on the struct;
impl Philosopher {
    // we define an "associated function" called new,
    // that takes a &str and returns a Philosopher.
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    // Methods take an explicit `self` parameter;
    // distinguishing it from the static method (or "associated function")
    fn eat(&self) {
        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    // a `Vec<T>`, or a vector, is a growable array type.
    let philosophers = vec![
        Philosopher::new("Mikael Stanne"),
        Philosopher::new("Jesper Strömblad"),
        Philosopher::new("Glenn Ljungström"),
        Philosopher::new("Anders Fridén"),
        // Can create Philosopher without the `new` method
        Philosopher { name: "Niclas Engelin".to_string() },
    ];

    // iterate over the vector, getting a reference to each philosopher
    for p in &philosophers {
        p.eat();
    }
}
