/*
   The Borrow Checker
   - The Rust compiler has a borrow checker that compares scopes to determine whether
     all borrows are valid.
   
   - Example in main.rs
   - Here we've annotated the lifetime of r with 'a and the lifetime of x with 'b
   - The inner 'b block is much smaller than the other 'a lifetime block
   - At compile time, Rust compares the size of the two lifetimes and sees the r has
     a lifetime of 'a but that it refers to memory with a lifetime of 'b
   - The program is rejected because 'b is shorter than 'a:the subject of the reference
     doesn't live as long as the reference.
*/