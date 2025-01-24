// Lifetime

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}  // 'a is a lifetime parameter that ensures s1, s2, and the return reference all have the same valid lifetime

fn main() {
    let str1 = String::from("Hello");
    let str2 = String::from("World!");

    let result = longest(&str1, &str2);
    println!("The longest string is: {}", result);
}
