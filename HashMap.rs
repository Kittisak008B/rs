use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new(); // 1. Create a new HashMap

    map.insert("name", "Sam");   // 2. Insert key-value pairs
    map.insert("age", "30");
    map.insert("city", "New York");
    map.insert("job", "Engineer");

    if map.contains_key("age") {  // 3. Check if a key exists using contains_key
        println!("Age key exists.");
    }

    if let Some(name) = map.get("name") {  // 4. Access a value using get (returns an Option)
        println!("Name: {}", name); //Name: Sam
    }

    // 5. Update a value for an existing key
    map.insert("age", "31"); // Update the age value to 31
    println!("Updated Age: {:?}", map.get("age")); //Updated Age: Some("31")

    // 6. Remove a key-value pair using remove
    map.remove("city");
    println!("After removing 'city': {:?}", map); //After removing 'city': {"name": "Sam", "age": "31", "job": "Engineer"}

    // 7. Check the size of the HashMap
    println!("Size of map: {}", map.len()); //Size of map: 3

    // 8. Check if the HashMap is empty
    if !map.is_empty() {
        println!("The map is not empty."); //The map is not empty.
    }

    // 9. Iterate over all key-value pairs
    println!("Key-Value Pairs:");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    // Key-Value Pairs:
    // name: Sam
    // age: 31
    // job: Engineer

    // 10. Clear all elements from the map
    map.clear();
    println!("After clearing: {:?}", map); //After clearing: {}

    // 11. Insert only if key does not exist
    if !map.contains_key("country") {
        map.insert("country", "USA");
    }
    match map.get("country") {  // Safely print the value of "country" if it exists
        Some(value) => println!("Country entry: {}", value), //Country entry: USA
        None => println!("Country key not found"),
    }

    // 12. Update the value if key exists
    if let Some(entry) = map.get_mut("country") {
        *entry = "Canada";
    }
    match map.get("country") { // Safely print the updated value of "country"
        Some(value) => println!("Updated Country entry: {}", value), //Updated Country entry: Canada
        None => println!("Country key not found"),
    }
}
