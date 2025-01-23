// Message Passing in Rust 

// Multiple Producer, Single Consumer (MPSC)  mpsc::channel() function creates a channel with two components:
// Sender (tx): Used to send messages.
// Receiver (rx): Used to receive messages

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel(); // Create a channel (tx: sender, rx: receiver)
  
    // Spawn a thread and move ownership of tx inside the closure
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "Rust", "thread!"];
        for msg in messages {
            tx.send(msg.to_string()).unwrap(); // Convert &str to String and send
            thread::sleep(Duration::from_millis(500));
        }
    });
    // The main thread receives messages
    for received in rx {
        println!("Main thread received: {}", received);
    }
}
