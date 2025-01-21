trait Sound {
    fn make_sound(&self);
}
trait Move {
    fn move_around(&self);
}
// Function that accepts a generic type implementing both the Sound and Move traits
fn animal_behavior<T: Sound + Move>(animal: &T) {
    animal.make_sound();
    animal.move_around();
}

struct Dog {
    name: String,
}
impl Sound for Dog {
    fn make_sound(&self) {
        println!("{} says: Woof! Woof!", self.name);
    }
}
impl Move for Dog {
    fn move_around(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let dog1 = Dog { name: String::from("Sam") };
    animal_behavior(&dog1); 

    let dog2 = Dog { name: String::from("Diddy") };
    animal_behavior(&dog2);
    animal_behavior(&dog2);
}
