use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create an Atomic Reference Counted Mutex containing a u64
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    // Number of threads
    let num_threads = 10;

    // Spawn threads
    for _ in 0..num_threads {
        // Clone the Arc to share ownership across threads
        let counter = Arc::clone(&counter);

        // Create a new thread
        let handle = thread::spawn(move || {
            // Lock the mutex to get access to the u64 value
            let mut num = counter.lock().unwrap();

            // Increment the u64 value
            *num += 1;
        });

        // Collect thread handles
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}
