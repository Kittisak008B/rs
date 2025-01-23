// Multiple producers can send data using cloned Senders.
// A single receiver (Receiver) gets all messages in FIFO order per sender.
// Due to concurrency, messages from different senders may interleave based on thread execution timing.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();
    drop(tx); // Drop the original sender to close the channel when producers finish

    // First producer
    thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("Producer 1: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Second producer
    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("Producer 2: Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // Receiver in the main thread
    for received in rx {
        println!("Received: {}", received);
    }
}
