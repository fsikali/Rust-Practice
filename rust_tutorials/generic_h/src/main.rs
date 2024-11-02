/*
   Generic Data Types In Enum Definitions
   
*/

#![allow(unused)]

fn main() { 
    enum Result<T, E> { 
        Ok(T),
        Err(E),
    }
}

/*
   The Result enum is generic over two types, T and E, and has two variants: Ok, 
   which holds a value of type T, and Err, which holds a value of type E.
   
   This definition makes it convenient to use the Result enum anywhere we have an
   operation that might succeed(return a value of some type T) or fail (return an
   error of some type E). 

   N/B - When you recognize situations in your code with multiple struct or enum 
         definitions that differ only in the types of the values they hold, you can
         avoid duplication by using generic types instead.
*/

