// Example - Creating a binary create (packages, modules and crates)

// The crate's directory, also named backyard, contains these files and directories
// The crate root file in this case is src/main.rs, and it contains; 

use crate::garden::vegetables::Asparagus; 

pub mod garden; 

fn main() { 
    let plant = Asparagus{}; 
    println!("I'm growing{:?}!", plant); 
} 

// The pub mode garden; line tells the compiler to include the code it finds in src/garden.rs, 
// which is ; pub mod vegetables 

// Here, pub mod vegetables; means thde code in src/garden/vegetables.rs in included too. That code is: 
// #[derive(Debug)]
// pub struct Asparagus {}
//




