/*
   Generic Data Types In Struct Definitions

   To define a Point struct where x and y are both generics but could have 
   different types, we can use multiple generic type parameters.
   
   We change the definition of Point to be generic over types T and U where x is
   of type T and y is of type U

 */

 struct Point<T, U> { 
    x: T,
    y: U,
 }

 fn main() { 
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
 } 


 