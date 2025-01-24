/*
   Generic Data Types In Enum Definitions
*/

#![allow(unused)] 

fn main() { 
    enum Option<T> { 
        Some(T),
        None,
    } 

} 

/*
   Option<T> enum is generic over type T and has two variants: 
   Some, which holds one value of type T, and a None variant that doesn't hold,
   any value.
   
   By using the Option<T> enum, we can express the abstract concept of an optional
   value, and because Option<T> is generic, we can use this abstraction no matter
   what the type of the optional value is.

*/ 
