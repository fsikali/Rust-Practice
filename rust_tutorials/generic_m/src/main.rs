/* 
   Generic Data Types - Performance of Code Using Generics 

   The monomorphized version of the code looks similar to the following(the compiler
   uses different names than what we're using here for illustration).
*/

enum Option_i32 { 
    Some(i32), 
    None,
} 

enum Option_f64 {
    Some(f64),
    None,
} 

fn main() { 
    let integer = Option_i32::Some(5); 
    let float = Option_f64::Some(5.0);
} 

/* 
   The generic Option<T> is replaced with the specific definitions created by the compiler
   Because Rust compiles generic code into code that specifies the type in each instance,
   we pay runtime cost for using generics. 
   When code runs, it performs just as it would if we had duplicated each definition by hand.
   The process of monomorphization makes Rust's generics extremely efficient at runtime.
 
*/