/*
   - In this section, we try to implement the longest function and
     therefore it doesn't compile
 
   - The help test reveals that the return type needs a generic lifetime parameter on it
     because Rust can't tell whether the reference being returned refers to x or y. 
   - Actually, we don't know either, because the if block in the body of this function
     returns a reference to x and the else block returns a reference to y. 
   
   - When we're defining this function, we don't know the concrete values that will be passed
     into this function, so we don't know whether the if case or the else case will execute.

   - We also don't know the conrete lifetimes of the references that will be passed in, so we
     can't look at the scopes to determine whether the reference we return will always be valid

   - The borrow checker can't determine this either, because it doesn't know how the lifetimes of x
     and y relate to the lifetime of the return value.

   - To fix this error, we'll add generic lifetime parameters that define the relationship between 
     the references so the borrow checker can perform its analysis.
*/