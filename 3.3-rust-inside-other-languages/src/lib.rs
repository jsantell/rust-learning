use std::thread;

#[no_mangle] // an attribute "no_mangle" that says don't change the name of the function
             // in the compiled output.
// `pub` means this function should be callable from outside the module
// `extern` means that this should be able to be called from C.
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in (0..5_00_00) {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
            h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
}
