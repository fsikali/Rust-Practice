/*
   Generic Data Types - Performance of Code Using Generics

   Is there a runtime cost when using generic type:
   - Using generic type won't make your program run any slower than it would with
     concrete types
   - Rust accomplishes this by performing monomorphization of the code using generics
     at compile time.
   - Monomorphization is the process of turning generic code into specific code by filling
     in the concrete types that are used when compiled.
   - In this process, the compiler does the opposite of the steps we used to create the generic
     function.
   
   - Example: The compiler looks at all the places where generic code is called and generates
              code for the concrete types generic code called with.

*/ 

// Using the standard library's generic Option<T> enum 

#![allow(unused)] 
fn main() { 
  let integer = Some(5); 
  let float = Some(5.0);
} 

/*
   When Rust compiles this code, it performs monomorphization
   The compiler reads the values that have been used in Option<T> instances and
   identifies two kinds of Option<T> : one is i32 and the other is f64. 
   As such it expands the generic definition of Option<T> into two definitions 
   specialized to i32 and f64, thereby replacing the generic definition with the
   specific ones. 

   
*/