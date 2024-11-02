/*
    Generic Data Types In Method Definitions:
    We can implement method on structs and enums and use generic types in 
    their definitions, too

    Example - Implementing a method named x on the Point<T> struct that will return
              a reference to the x field of type T

*/

struct Point<T> { 
    x: T;
    y: T,
} 

impl<T> Point<T> { 
    fn x(&self) -> &T { 
        &self.x
    }
} 

fn main() { 
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

/* 
  Here, we've defined a method named x on Point<T> that returns a reference to the data
  in the field x. 

  By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets
  in Point is a generic type rather than a concrete type

  
 */