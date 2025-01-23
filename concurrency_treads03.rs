// smart pointers in threads

// Arc<T> (Atomic Reference Counted)
// Arc is a thread-safe version of Rc (Reference Counted). 
// It provides reference counting for shared ownership of data across multiple threads


// Mutex<T> (Mutual Exclusion)
// Mutex is a synchronization primitive that ensures only one thread can access the data at a time. 
// It is used when you need to mutate or modify shared data safely across multiple threads.

// Mutex<T> uses a "lock" to control access to data. A thread must lock the mutex before accessing the data. 
// If another thread holds the lock, the thread must wait. After using the data, the thread unlocks the mutex so others can access it.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Use Arc to share Mutex between threads

    let mut handles = vec![]; // Create a vector to hold the handles of the spawned threads

    // Spawn 10 threads that each increment the counter
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap(); // Lock the mutex and increment the counter
            *num += 1;
        });

        handles.push(handle); // Store the thread handle
    }
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final counter: {}", *counter.lock().unwrap()); // Print the final value of the counter
}
