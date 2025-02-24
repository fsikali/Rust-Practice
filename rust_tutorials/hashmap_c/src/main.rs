/*
- We can iterate over each key/value pair in a hashmap in a similar manner we do with vectors
  using a for loop:
*/

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); 

    scores.insert(String::from("Blue"), 10); 
    scores.insert(String::from("Yellow"), 50); 

    for (key, value) in &scores { 
        println!("{key}: {value}");
    }
}
