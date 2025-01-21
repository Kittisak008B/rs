// Standard Traits

// Trait	    Purpose
// Clone	    Allows deep copying of structs and enums.
// Debug	    Enables {:?} formatting for debugging.
// PartialEq	Enables == and != comparisons.
// Eq	        Ensures total equality (used with PartialEq).
// PartialOrd	Enables <, >, <=, >= but allows incomplete comparisons (e.g., NaN).
// Ord	        Enables total ordering (requires PartialOrd).
// Default	    Provides a default instance with Struct::default().

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let user1 = User::default(); // Using Default trait
    println!("{:?}", user1); // Debug trait in action

    let user2 = User { id: 1, name: "Sam".to_string() };
    let user3 = user2.clone(); // Clone trait in action

    // PartialEq trait
    if user2 == user3 {
        println!("Users are equal");
    }

    // Ord trait
    let user4 = User { id: 2, name: "Bob".to_string() };
    if user2 < user4 {
        println!("User2 comes before User4");
    }
}
