/* 
    Generic Data Types in Struct Definitions
    - We can also definee structs to use a generic type parameter in one or more fields
      using the <> syntax.
    
    Example: A Point<T> struct that holds x and y values of types T 

 */

struct Point<T> { 
    x:T, 
    y:T,
} 

fn main() { 
    let integer = Point { x: 5, y: 10 }; 
    let float = Point { x: 1.0, y: 4.0 };
} 

// N/B - The fields x and y are both the same type, whatever that type may be
// If we create an instance of a Point<T> that has values of different types, our
// code won't compile
// This error is called a type mismatch error

