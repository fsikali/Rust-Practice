/*
- Using the List type to store the list 1, 2, 3 would look like this:
- This code does not yet compile
*/

// Example: Using the List enum to store the list 1, 2, 3

enum List { 
    Cons(i32, List), 
    Nil,
}

use crate::List::{Cons, Nil}; 

fn main() { 
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
