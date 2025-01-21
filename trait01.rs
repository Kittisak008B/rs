// Traits: Defining Shared Behavior
// Trait in Rust is similar to an interface in other languages. It defines a set of methods that a type must implement. 
// Traits enable polymorphism by allowing different types to share the same behavior

trait Greet {
    fn say_hello(&self); // A method that must be implemented
}

struct Person {
    name : String ,
}

// Implement the `Greet` trait for `Person`
impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, I am {}" , self.name);
    }
}

fn main(){
    let person1 = Person{
        name : String::from("Diddy") ,
    };
    person1.say_hello(); // Hello, I am Diddy

}
