// The code a user of our crate will be able to write when using our procedural macro 

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro; 

#[derive(HelloMacro)] 
struct Pancakes; 

fn main() { 
    Pancakes::hello_macro();
}

