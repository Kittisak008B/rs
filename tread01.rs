// Concurrency 

// Concurrency is the ability to write programs that execute multiple tasks independently. 
// Rust ensures safety and efficiency using its ownership system, type system, and compile-time checks
// to prevent issues like data races, deadlocks, and dangling pointers—all without relying on a garbage collector.

// key concurrency concepts in Rust:
// Ownership and Borrowing    : Ensures safe data access and prevents data races through compile-time checks.
// Threads                    : Rust allows spawning threads with std::thread::spawn for concurrent execution.
// Message Passing            : Threads communicate using channels (mpsc) to avoid shared state and data races.
// Shared State Concurrency   : Arc (atomic reference counting) and Mutex provide safe shared access to data across threads.
// Asynchronous Programming   : Uses async/await for non-blocking tasks, efficiently running concurrent operations.
// No Garbage Collector       : Rust enforces memory safety without the overhead of garbage collection, improving performance.
// These concepts work together to provide safe, efficient, and concurrent execution without runtime overhead.

use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("spawned thread! Iteration: {}", i);
            thread::sleep(Duration::from_millis(500)); // Simulate work
        }
    });

    // Main thread continues execution
    for i in 1..=5 {
        println!("main thread! Iteration: {}", i);
        thread::sleep(Duration::from_millis(200));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
