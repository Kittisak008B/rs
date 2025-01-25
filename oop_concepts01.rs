// OOP concepts in rust

// Encapsulation
// It hides the internal details of the implementation, exposing only what's necessary
// In Rust, encapsulation is achieved using struct for data storage and impl blocks for methods.

struct Car {
    brand: String,
    speed: u32,
}

impl Car {
    fn new(brand: &str, speed: u32) -> Self {
        Self {
            brand: brand.to_string(),
            speed,
        }
    }
    fn accelerate(&mut self, amount: u32) {
        self.speed += amount;
    }
    fn get_speed(&self) -> u32 {
        self.speed
    }
}

fn main() {
    let mut my_car = Car::new("Tesla", 100);
    my_car.accelerate(20);
    println!("Car speed: {}", my_car.get_speed());
    println!("Car brand: {}", my_car.brand);
}
