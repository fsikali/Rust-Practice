/*
- Enum definition for a cons list
- N/B: This code won't compile yet because the List type doesn't have a known size
*/

// Example: The first attempt at defining an enum to represent a cons list data structure
//          of i32 values

enum List { 
    Cons(i32, List),
    Nil,
}

fn main() {}
