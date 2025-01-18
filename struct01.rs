// struct is used to define custom data types by grouping related fields together

#[derive(Debug)] // Enables Debug printing for the struct
struct Car {
    brand: String,
    year: u32,
}

fn main() {
    let my_car = Car {
        brand: String::from("Tesla"),
        year: 2010,
    };
  
    println!("Car brand: {} , Year: {} \n", my_car.brand , my_car.year);
    println!("{:?}", my_car);   // Prints entire struct in a single line 
    println!("{:#?}", my_car);  // Pretty-printed 
}
