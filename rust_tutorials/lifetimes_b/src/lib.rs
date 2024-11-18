/*
   The Borrow Checker

   - Example in main.rs
   - Here, x has the lifetime 'b, which in this case is longer than 'a.
   - This means r can reference x because Rust knows that the reference in r
     will always be valid while x is valid.
     
*/