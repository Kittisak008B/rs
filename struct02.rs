// Implementing Methods in struct

struct Car {
    brand: String,
    year: u32,
}

impl Car {
    // Method to get the car's details as a string
    fn details(&self) -> String {
        format!("{} - {}", self.brand, self.year)
    }
    // Method to check if the car is new
    fn is_new(&self) -> bool {
        self.year >= 2023
    }
}

fn main() {
    let my_car = Car {
        brand: String::from("Ford"),
        year: 2010,
    };

    println!("Car Details: {}", my_car.details());
    println!("Is the car new? {}", my_car.is_new());
}
