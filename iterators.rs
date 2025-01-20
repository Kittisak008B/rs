// iterators

fn main() {
    let numbers : Vec<i32> = vec![10, 21, 30, 41, 50];

    for (i , num) in numbers.iter().enumerate() {  
        println!("i:{} num:{}", i , num);
    }


    let doubled_even_numbers: Vec<i32> = numbers.iter() // .iter() gives references to each element (&10, &21, &30, &41, &50)
        .filter(|x :&&i32| *x % 2 == 0)   // Filter even numbers
        .map(|x :&i32| x * 2)             // Double the even numbers
        .collect();    // Collect into a new vector

    println!("Doubled even numbers: {:?}", doubled_even_numbers);
}
