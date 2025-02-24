/*
--- Accessing Values in Hash Map
- We can get a value out of the hash map by providing its key to get method
*/

// Example: Accessing the score for the Blue team stored in the hash map

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue"); 
    let score = scores.get(&team_name).copied().unwrap_or(0); 

    println!("{team_name}: {score}");
}
