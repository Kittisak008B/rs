trait Animal {
    fn make_sound(&self); // Method that must be implemented

    // Default method implementation
    fn eat(&self) {
        println!("The animal is eating.");
    }
}

struct Dog {
    name: String,
}
// Implement the `Animal` trait for `Dog`
impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says: Woof!", self.name);
    }
}


struct Cat {
    name: String,
}
// Implement the `Animal` trait for `Cat`
impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says: Meow!", self.name);
    }
}

// Function that takes any type that implements `Animal` trait
fn animal_sound<T : Animal>(x :&T){
    x.make_sound();
}

fn main() {
    let dog = Dog {
        name: String::from("Sam"),
    };
    dog.make_sound(); // Sam says: Woof!
    dog.eat(); // The animal is eating.

    let cat = Cat {
        name: String::from("Nancy"),
    };
    cat.make_sound(); // Nancy says: Meow!
    cat.eat(); // The animal is eating.

    animal_sound(&dog); // Sam says: Woof!
    animal_sound(&cat); // Nancy says: Meow!
}
