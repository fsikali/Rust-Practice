/* 
   Validating References with Lifetimes 
   - Lifetimes are another type of generic
   - Rather than ensuring that a type has a behaviour we want, lifetimes ensure that
     references are valid as long as we need them to be.
   - Every reference in Rust has a lifetime, which is the scope for which that reference is valid
   - Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred
   - We must only annotate types when multiple types are possible
   - In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a 
     few different ways.
   - Rust required us to annotate the relationships using generic lifetime parameters to ensure the actual
     references used at runtime will definitely be valid 

   Preventing Dangling References with Lifetimes
   - The main aim of lifetimes is to prevent dangling references, which cause a program to reference
     data other than the data it's intended to reference.

   - Example in main.rs
   - The outer scope declares a variable named r with no initial value,and the inner scope declares
     a variable named x with the initial value of 5.
   - Inside the inner scope,we attempt to set the value of r as a reference to x. 
   - Then the inner scope ends, and we attemp to print the value in r.
   - This code won't compile because what the value r is referring to has gone out of scope
     before we try to use it.
   - The variable x doesn't "live long enougn".
   - The reason is that x will be out of scope when the inner scope ends on line 7
   - But r is still valid for the outer scope; because its scope is larger, we say that it 
     "lives longer".
   - If Rust allowed this code to work, r would be referencing memory that was deallocated when x
     went out of scope, and anythinh we tried to do with r wouldn't work correctly.
   - Rust determines that this code is invalid by using a borrow checker.
*/


