use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::new(); // Create a new empty VecDeque
    println!("Created empty deque: {:?}", deque);

    deque.push_back(1); // Adding elements to the back
    deque.push_back(2);
    deque.push_back(3);
    println!("After push_back: {:?}", deque); // Output: [1, 2, 3]

    deque.push_front(0); // Adding elements to the front
    println!("After push_front: {:?}", deque); // Output: [0, 1, 2, 3]

    // Peek at the front and back
    if let Some(front) = deque.front() {
        println!("Front element: {}", front);  // Output: Front element: 0
    }
    if let Some(back) = deque.back() {
        println!("Back element: {}", back);    // Output: Back element: 3
    }

    let removed_front = deque.pop_front(); // Remove elements from the front
    println!("Removed from front: {:?}", removed_front); // Output: Some(0)
    println!("Deque after pop_front: {:?}", deque); // Output: [1, 2, 3]

    let removed_back = deque.pop_back(); // Remove elements from the back
    println!("Removed from back: {:?}", removed_back);   // Output: Some(3)
    println!("Deque after pop_back: {:?}", deque);       // Output: [1, 2]

    // Check if a value is contained in the deque
    if deque.contains(&2) {
        println!("2 is in the deque.");  // Output: 2 is in the deque.
    } else {
        println!("2 is not in the deque.");
    }
    if deque.contains(&5) {
        println!("5 is in the deque.");
    } else {
        println!("5 is not in the deque.");  // Output: 5 is not in the deque.
    }

    // Check the length of the deque
    println!("Length of deque: {}", deque.len());  // Output: Length of deque: 2

    // Check if deque is empty
    println!("Is deque empty? {}", deque.is_empty()); // Output: Is deque empty? false

    // Clear the deque
    deque.clear();
    println!("Deque after clear: {:?}", deque); // Output: []

    // Adding elements again to demonstrate other methods
    deque.push_back(4);
    deque.push_back(5);
    deque.push_back(6);

    // Iterating over the deque
    println!("Iterating over deque:");
    for item in &deque {
        println!("{}", item);  // Output: 4, 5, 6
    }

    // Convert VecDeque to Vec
    let vec: Vec<i32> = deque.into();
    println!("Converted to Vec: {:?}", vec);  // Output: [4, 5, 6]
}
