// Enums represent multiple related values as different states in Rust

enum Message {
    Quit,                      // No data
    Move { x: i32, y: i32 },   // Struct-like variant
    Write(String),             // Tuple variant
    ChangeColor(u8, u8, u8),   // RGB color
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to position: ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color changed to RGB({}, {}, {})", r, g, b),
    }
}

fn main() {
    let msg1 = Message::Move { x: 10, y: 20 };
    let msg2 = Message::Write(String::from("Hello Rust!"));
    
    process_message(msg1);
    process_message(msg2);
}
