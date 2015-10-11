use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a new channel
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        // Clone is for use in another thread
        let tx = tx.clone();

        thread::spawn(move || {
            let answer = 42;
            tx.send(answer);
        });
    }

    // Let's test a panic in a thread
    let result = thread::spawn(move || {
        panic!("oops!");
    }).join();
    assert!(result.is_err());

    rx.recv().ok().expect("Could not receive answer");
}
