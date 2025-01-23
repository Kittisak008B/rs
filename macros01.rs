// Macros in Rust generate code at compile time. 
// When you use a macro, the Rust compiler expands the macro into actual code before compiling your program. 
// This expansion happens during the preprocessing phase of compilation, not at runtime

// Declarative Macros (macro_rules!) : define patterns that expand into code based on matching inputs.

// Procedural Macros : operate on Rust code itself, including:
// Custom Derives: Implement traits automatically.
// Attribute-like Macros: Define custom attributes.
// Function-like Macros: Operate like functions but with code input.


// macro_rules! greet {
//     () => {
//         println!("Hello, world!");
//     };
// }
// fn main() {
//     greet!();  // Expands to: println!("Hello, world!");
// }

macro_rules! calculate {
    // Pattern for adding two numbers
    ($x:expr, +, $y:expr) => {
        $x + $y
    };
    // Pattern for subtracting two numbers
    ($x:expr, -, $y:expr) => {
        $x - $y
    };
    // Pattern for multiplying two numbers
    ($x:expr, *, $y:expr) => {
        $x * $y
    };
}
fn main() {
    println!("5 + 3 = {}", calculate!(5, +, 3));  
    println!("10 - 4 = {}", calculate!(10, -, 4)); 
    println!("4 * 6 = {}", calculate!(4, *, 6));  
}
