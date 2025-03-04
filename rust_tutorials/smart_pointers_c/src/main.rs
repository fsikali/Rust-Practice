/*
- We change the definition of the List enum and the usage of the List
*/

// Example: Definition of LIst that uses Box<T> in order to have a known size

enum List { 
    Cons(i32, Box<List>), 
    Nil,
}

use crate::List::{Cons, Nil}; 

fn main() { 
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
