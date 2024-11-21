/*
   Example explanation:
   - Here,even though we've specified a lifetime parameter 'a for the return type, this implementation
     will fail to compile because the return value lifetime is not related to the lifetime of the parameters
     at all.
   - The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
   - We're also trying to return a reference to result from the function.
   - There is no way we can specify lifetime parameters that would change the dangling reference, and Rust
     won't let us create a dangling reference.
   - In this case, the best fix would be return an owned data type rather than a reference so the calling 
     function is then responsible for cleaning up the value.
   - Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values 
     functions.
   - Once they're connected, Rust has enough information to allow memory-safe operatins and disallow operations
     that would create dangling pointers or otherwise violate memory safety.
*/