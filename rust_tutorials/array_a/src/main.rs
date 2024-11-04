// Invalid Array Element Access  

use std::io; 

fn main() { 
    let a = [1, 2, 3, 4, 5]; 

    println!("Please enter an array index."); 

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line"); 

    let index: usize = index 
        .trim()
        .parse()
        .expect("Index entered was not a number"); 

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

/* 
   N/B - This code compiles successfully.
       - If you run this code using cargo run and enter 0, 1, 2, 3, or 4 the
         program will print out the corresponding value at that index in the array
       - If you instead enter a number past the end of the array you'll get a runtime error
*/