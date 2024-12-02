/*
   Lifetime Elision
   - Every reference has a lifetime and that you need to specify lifetime parameters for functions
     or structs that use references.
   - Example in main.rs

   - Example Explanation:
   - In early versions (pre-1.0) of Rust, this code wouldn't have compiled because every reference
     needed an explicit lifetime.
    
*/ 

// How the function signature would have been written 
fn first_word<'a>(s: &'a str) -> &'a str {}

