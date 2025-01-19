// enum Option<T> {
//     Some(T),    //Some: Contains a value
//     None,       //None: Represents the absence of a value.
// }

fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Sam".to_string()),
        2 => Some("Diddy".to_string()),
        _ => None,   // No user found
    }
}

fn main() {
    let user = find_user(2);
    match user {
        Some(name) => println!("User found: {}", name),
        None => println!("User not found"),
    }
}
