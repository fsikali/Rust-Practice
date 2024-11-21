/*
   Example: 
   - Here we show that the lifetime of the reference in result must be the smaller lifetime of the two
     arguments.
   - We'll move the declaration of the result variable outside the inner scope but leave the assignment
     of the value to the result variable inside the scope with string2.
   - Then we'll move the println! that uses result to outside the inner scope, after the inner scope has
     ended 
   - Example in main.rs

   - Error explanation
   - The error shows that for result to be valid for the println! statement, string2 would need to be
     valid until the end of the outer scope.
   - Rust knows this because we annotated the lifetimes of the function parameters and return values
     using the same lifetime parameter 'a.
   

   N/B 

   - As humans, we can look at this code and see that string1 is longer than string2 and therefore result
     will contain a reference to string1.
   - Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the 
     println! statement
   - However, the compiler can't see that the reference is valid in this case
   - We've told Rust that the lifetime of the reference returned by the longest function is the same as the
     smaller of the lifetimes of the references passed in.
   - Therefore, the borrow checker disallows the code as possibly having an invalid reference

*/