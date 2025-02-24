// Creating a new hashmap and inserting some keys and values
// One way to create an empty hash map is using new and adding elements with insert

use std::collections::HashMap;

fn main() { 
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);  

}
