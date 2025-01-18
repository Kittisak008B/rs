use std::io; 
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("type: {}" , type_name::<T>());
}

fn main() {
    println!("Hello World!");

    // Reads input from the user
    let mut input : String = String::new() ;
    println!("Enter some text: ");
    io::stdin().read_line(&mut input).expect("Failed to readline");
    println!("You entered :{}", input.trim());
  

    let mut input2 : String = String::new();
    println!("Enter a floating-point number: ");
    io::stdin().read_line(&mut input2).expect("Failed to readline"); // Read input from the user
    let input2 = input2.trim(); // Trim the input to remove any extra spaces or newline characters
    // Convert the string to a floating-point number (f64)
    let number: f64 = match input2.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid floating-point number.");
            return;
        }
    };
    println!("You entered: {}", number);

    // Signed   -> i8 range Minimum: -2^7 = -128 , Maximum: 2^7 - 1 = 127
    // Unsigned -> u8 range Minimum: 0 , Maximum: 2^8 - 1 = 255
    let res1:i32 = -2i32.pow(7); // -2^7
    let res2:i32 = (1 << 7) -1; // 2^7 - 1
    let res3:i32 = res1 + res2;
    let res4:i32 = i8::MAX as i32 ;
    let res5:i32 = i8::MIN as i32 ;
    println!("{} {} {} {} {}", res1 , res2 , res3 , res4 , res5);

    let t1 : i32 = 10 ;
    let t2 : bool = true ;
    let t3 : char = 'a' ;
    let t4 : f32 = 11.11 ;
    println!("{} {} {} {} " , t1 , t2 , t3 , t4);
    let msg1 :String = String::from("Hello, World!");
    let msg2 :String = "Hello, World!".to_string();
    let msg3 :&str = "Hello, World!";
    let msg4 :String = format!("test: {}, {}", t1, t2);
    println!("{} {} {} {}", msg1 , msg2 , msg3 , msg4);
    
    let num = 42;      
    let pi = 3.14;      
    let msg = "Hello";  
    let list = vec![1, 2, 3]; 
    print_type_of(&num);   
    print_type_of(&pi);    
    print_type_of(&msg);   
    print_type_of(&list);  

    let test_num:i32 = {
        let x:i32 = 3 ;
        x + 2
    };
    println!("{}" , test_num);
}
