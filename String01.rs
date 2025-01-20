// ASCII (American Standard Code for Information Interchange)
// Character Set : 128 characters (English letters, digits, punctuation)
// Byte Size : 1 byte per character

// UTF-8 (Unicode Transformation Format - 8-bit)
// Character Set : Over 1.1 million characters (supports all languages, symbols, emojis)
// Byte Size : 1 to 4 bytes per character

fn main() {
    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);


    let s4 = vec!["1", "2", "3"].join("/");
    println!("{}", s4);


    let s_ascii = String::from("Hello Rust!"); // ASCII
    if let Some(slice) = s_ascii.get(0..5) {
        println!("{}", slice); 
    } else {
        println!("Invalid slice!");
    }


    let s_uft8 = "नमस्ते"; // Hindi word, UTF-8 encoding
    // Print each byte in UTF-8 encoding
    for byte in s_uft8.as_bytes() {
        print!("{:x} ", byte); 
    }
    println!(); // To move to the next line

    // Print Unicode scalar values (code points) of each character
    for c in s_uft8.chars() {
        print!("{:x} ", c as u32); 
    }
}
