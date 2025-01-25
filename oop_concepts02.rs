// OOP concepts in rust

// Composition and trait inheritance
// In Rust, you can define shared behavior through traits and implement these traits for different types
// trait inheritance allows one trait to extend another, offering a way to define shared behavior

trait Vehicle {
    fn get_speed(&self) -> u32;
}
trait Electric: Vehicle {
    fn charge_battery(&self);
}

struct Tesla {
    speed: u32,
}

impl Vehicle for Tesla {
    fn get_speed(&self) -> u32 {
        self.speed
    }
}
impl Electric for Tesla {
    fn charge_battery(&self) {
        println!("Tesla battery charging...");
    }
}

fn main() {
    let my_tesla = Tesla { speed: 120 };
    
    println!("Tesla speed: {} km/h", my_tesla.get_speed()); // Using the Vehicle trait method

    my_tesla.charge_battery(); // Using the Electric trait method
}
