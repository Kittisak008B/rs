// Enum with Methods

#[derive(Debug)] // Enables Debug printing for TrafficLight
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // Method to get the duration of each light
    fn duration(&self) -> u32 {
        match self {
            Self::Red => 60,     // Using Self:: to refer to the enum     
            Self::Yellow => 5,   
            Self::Green => 30,   
        }
    }
    // Method to check if it's safe to go
    fn can_go(&self) -> bool {
        match self {
            Self::Green => true,
            _ => false,
        }
    }
}

fn main() {
    let lights = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];

    for light in &lights {
        println!("{:?} light duration: {} seconds | Can go? {}" , light , light.duration() , light.can_go());
    }
}
