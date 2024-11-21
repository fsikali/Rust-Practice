/*
   Thinking in Terms of Lifetimes
   - The way in which you need to specify lifetime parameters depends on what your function is doing.
   - If we changed the implementation of the longest function to always return the first parameter rather
     than the longest string slice, we wouldn't need to specify a lifetime on the y parameter
   

   - Example explanation:
   - We've specified a lifetime parameter 'a for the parameter x and the return type, but not for the
     parameter y, because the lifetime of y does not have any relationship with the lifetime of x or
     the return value
   - When returning a reference from a function, the lifetime parameter fot the return type needs to match
     the lifetime parameter for one of the parameters
   - If the reference returned does not refer to one of the parameters, it must refer to a value created
     within this function
   - However, this would be a dangling reference because the value will go out of scope at the end of the 
     function.

*/