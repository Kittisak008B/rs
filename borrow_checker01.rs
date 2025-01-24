// borrow checker is a core part of Rust’s ownership system that ensures memory safety without requiring a garbage collector. 
// It prevents data races, dangling pointers, and use-after-free bugs by enforcing ownership, borrowing, and lifetimes at compile time.

// Each value in Rust has a single owner.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moves to s2, s1 is no longer valid

    // println!("{}", s1);  // ❌ ERROR: s1 was moved!
    println!("{}", s2);  // ✅ Works fine
} // Once an owner goes out of scope, Rust automatically frees the memory.

// You can have only ONE mutable reference at a time.
fn main2() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // ❌ ERROR: Cannot borrow `s` mutably more than once

    println!("{}", r1);
}

// You can have multiple immutable references, but NO mutable reference at the same time.
fn main3() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;  // ❌ ERROR: Cannot have a mutable reference while immutable references exist

    println!("{}, {}", r1, r2);
}
