use std::sync::{Mutex, Arc};
use std::thread;

// Create a struct that represents a philosopher
// Store a property "name" of type `String` -- generally
// better for structures to own its own data, rather than using
// references
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

// We define things on the struct;
impl Philosopher {
    // we define an "associated function" called new,
    // that takes a &str and returns a Philosopher.
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    // Methods take an explicit `self` parameter;
    // distinguishing it from the static method (or "associated function")
    fn eat(&self, table: &Table) {
        // Use an underscored variable name to indicate
        // to rust that we're not going to use these variables,
        // and we know that, so don't show an error
        //
        // Accesses a mutex from the fork table, and calls lock, which
        // either locks the thread, or waits if the thread is unlocked
        //
        // Just use unwrap() here as well since we do not expect the threads
        // to panic and fail
        let _left = table.forks[self.left].lock().unwrap();
        // The locks release when _left and _right go out of scope
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    // Arc stands for "atomic reference count", which we need
    // to share Table across multiple threads
    // as we share it, the reference count will go up and when each thread ends, it will go back
    // down.
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    // a `Vec<T>`, or a vector, is a growable array type.
    let philosophers = vec![
        Philosopher::new("Mikael Stanne", 0, 1),
        Philosopher::new("Jesper Strömblad", 1, 2),
        Philosopher::new("Glenn Ljungström", 2, 3),
        Philosopher::new("Anders Fridén", 3, 4),
        // Can create Philosopher without the `new` method
        // Notice how the last one doesn't match the pattern, otherwise we'd have deadlock
        Philosopher { name: "Niclas Engelin".to_string(), left: 0, right: 4 },
    ];

    // `let handles: Vec<_> =`
    // Explicitly annotating the type Vec<_>,
    // with type `_` which is a placeholder type
    // meaning "Rust will figure it out"
    //
    // `philosophers.into_iter().map(|p| {`
    // We create an iterator from our philosophers vector,
    // takes ownership of each philosophers to pass into our threads.
    // Map takes a closure so that thread::spawn is using the
    // correct `p` value.
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // clone() method on Arc<T> is what bumps up the ref count,
        // and decrements when falls out of scope. We want to track how many refs
        // to table exist across our threads. If we didn't have a count, we swouldn't
        // know how to deallocate it.
        let table = table.clone(); // shadow binding

        // Executes the closure passed into `thread::spawn` in
        // a new thread
        //
        // We annotate with `move` indicating that the closure
        // is going to take ownership of the values its capturing,
        // the `p` from the current closure.
        thread::spawn(move || {
            p.eat(&table);
        }) // nnote lack of semicolon; expression, returns return
        // values of thread::spawn calls which are handles to those threads
    }).collect(); // makes them into some kind of collection, this is why we needed
                  // `Vec<_>`

    for h in handles {
        // `join()` blocks execution until the thread has completed execution.
        // So all programs will complete their work before moving on to the next
        // handle in the loop
        h.join().unwrap();
    }
}
