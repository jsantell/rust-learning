use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /*
    // Doing this would result in each thread having
    // a reference to `data`, and since each thread becomes
    // its owner, we have three owners, resulting in error:
    // 8:17 error: capture of moved value: `data`
    let mut data = vec![1, 2, 3];
    for i in 0..3 {
        thread::spawn(move || {
            data[i] += 1;
        });
    }
    */

    // We need to ensure that each owner can mutate the value
    // but only one thread at a time can mutate what's inside,
    // which is our Mutext<T> type.

    // Mutex has a `lock` method
    // fn lock(&self) -> LockResult<MutexGuard<T>>
    // Because Send is not implemented for MutexGuard<T>, we can't transfer the guard across thread
    // So we wrap in `Arc<T>`
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        // Use `.clone()`, which increases refcount on Arc
        // this handle is then moved into the new thread
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data
                           // `lock()` returns a `Result<T, E>`,
                           // in this example we assume it doesn't fail, so just unwrap
                           .lock()
                           .unwrap();
            data[i] += 1;
        });
    }
    thread::sleep_ms(50);
}
