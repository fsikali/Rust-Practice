/*
--- Overwriting a Value
*/

// Example: Replacing a value stores with a particular key

use std::collections::HashMap; 

fn main() { 
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); 

    println!("{:?}", scores);
}
