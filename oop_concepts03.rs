// OOP concepts in rust

// Polymorphism
// In Rust, polymorphism is achieved through traits, 
// allowing different types to implement the same trait and behave differently when the trait's methods are called
// This is commonly done with dynamic dispatch 

trait Animal {
    fn make_sound(&self);
}
struct Dog;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}
struct Cat;
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}
fn make_noise(animal: &dyn Animal) {
    animal.make_sound();  // Rust uses dynamic dispatch to call the correct method based on the actual type of the object passed at runtime (Dog or Cat)
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    make_noise(&dog);
    make_noise(&cat);
}
// dyn is used to create a trait object that can point to any type that implements the trait.

// Dynamic Dispatch: When you pass &dog or &cat to make_noise, Rust doesn't know at compile time which type will be passed 
// (it could be Dog, Cat, or any other type that implements the Animal trait). 
// So, it uses dynamic dispatch to figure out the actual method to call at runtime based on the object’s type.
