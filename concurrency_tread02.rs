// Moving Ownership into Threads

use std::thread;
use std::time::Duration;

fn main() {
    let message = String::from("Hello, Rust!");

    // Move message before the loop
    let handle = thread::spawn(move || {
        let msg = message; // Move message once
        for i in 1..=5 {
            println!("spawned thread! Iteration: {} {}", i, msg);
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
