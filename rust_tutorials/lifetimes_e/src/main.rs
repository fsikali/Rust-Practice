/*
  Examples: 
  - A reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime
    parameter named 'a, and a mutable reference to an i32 that also has the lifetime 'a.
*/ 

&i32             // a reference 
&'a i32          // a reference with an explicit lifetime
&'a mut i32      // a mutable reference witth an explicit lifetime

