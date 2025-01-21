// Generics:
// You must specify the type (T, U, etc.) when using or implementing a trait.
// Offers flexibility, allowing different types to be used in different places.
// Ideal for situations where you need high flexibility or changing types.

// Associated Types:
// Define the related type directly within the trait.
// The type is "tied" to the trait when implemented, reducing the need to specify the type repeatedly.
// Ideal when the trait has a specific type that doesn't change across implementations.

trait Shape {
    type AreaType; // Associated type: the area type of the shape
    fn area(&self) -> Self::AreaType; // Method to compute the area
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    type AreaType = f64; // Associated type for Rectangle is f64 
    fn area(&self) -> Self::AreaType {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}
impl Shape for Circle {
    type AreaType = f64; // Associated type for Circle is f64
    fn area(&self) -> Self::AreaType {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let rectangle = Rectangle { width: 5.0, height: 10.0 };
    println!("Rectangle area: {}", rectangle.area());

    let circle = Circle { radius: 3.0 };
    println!("Circle area: {}", circle.area());
}
