// smart pointers
// Box<T> : allocates a value on heap. Useful for recursive data structures
// Rc<T> : enables multiple pointers to share ownership
// RefCell<T> : allows interior mutability

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let count = Rc::new(RefCell::new(0)); //Rc + RefCell for shared mutable state
  
    // Clone Rc<T> to create multiple references to the same counter
    let count_clone1 = Rc::clone(&count);
    let count_clone2 = Rc::clone(&count);

    // Modify the counter using RefCell<T>
    *count_clone1.borrow_mut() += 2;
    *count_clone2.borrow_mut() += 3;

    println!("Reference count: {}", Rc::strong_count(&count));  // Reference count: 3
    println!("value : {}" , count.borrow());                    // value : 5
    println!("value : {}" , *count_clone1.borrow());            // value : 5
    println!("value : {}" , *count_clone2.borrow());            // value : 5
}
